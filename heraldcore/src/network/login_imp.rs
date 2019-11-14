use super::*;
use std::ops::Deref;
use tokio::net::tcp::TcpStream;

/// Attempts to login to the server, spawning a long-lived thread to handle messages pushed from
/// the server.
///
/// Takes a callback as an argument that is called whenever a message is received.
pub async fn login<F, G>(mut f: F, mut g: G) -> Result<(), HErr>
where
    F: FnMut(Notification) + Send + 'static,
    G: FnMut(HErr) + Send + 'static,
{
    use login::*;

    if sodiumoxide::init().is_err() {
        eprintln!("failed to init libsodium - what are you doing");
        std::process::abort()
    }

    let uid = Config::static_id()?;
    let kp = Config::static_keypair()?;
    let gid = GlobalId {
        uid,
        did: *kp.public_key(),
    };

    let mut stream = Framed::new(TcpStream::connect(SERVER_TCP_ADDR.deref()).await?);

    stream.write_timed(&SignAs(gid)).await?;
    match stream.read_timed().await? {
        SignAsResponse::Sign(u) => {
            let token = LoginToken(kp.raw_sign_detached(u.as_ref()));
            stream.write_timed(&token).await?;

            match stream.read_timed().await? {
                LoginTokenResponse::Success => {}
                e => return Err(SignInFailed(e)),
            }
        }
        e => return Err(GIDSpecFailed(e)),
    }

    let ev = catchup(&mut stream).await?;

    CAUGHT_UP.store(true, Ordering::Release);
    let guard = scopeguard::guard((), |()| CAUGHT_UP.store(false, Ordering::Acquire));

    // clear pending
    for (tag, cid, content) in pending::get_pending()? {
        send_cmessage(cid, &content).await?;
        pending::remove_pending(tag)?;
    }

    // send read receipts, etc
    ev.execute(&mut f, &mut g).await?;

    tokio::spawn(async move {
        let comp: Result<(), HErr> = async move {
            let _guard = guard;
            loop {
                let ev = catchup(&mut stream).await?;
                ev.execute(&mut f, &mut g).await?;
            }
        }
            .await;
        comp.unwrap_or_else(|e| eprintln!("connection failed with message {}", e));
    });

    Ok(())
}

async fn catchup(stream: &mut Framed<TcpStream>) -> Result<Event, HErr> {
    use catchup::*;

    let mut ev = Event::default();

    while let Catchup::Messages(p) = stream.read_packeted().await? {
        let len = p.len() as u64;
        for push in p.iter() {
            match handle_push(push).await {
                Ok(e2) => ev.merge(e2),
                Err(e) => {
                    eprintln!("error while catching up, error was:\n{}", e);
                    ev.errors.push(e);
                }
            }
        }
        stream.write_timed(&CatchupAck(len)).await?;
    }

    Ok(ev)
}

async fn handle_push(push: &Push) -> Result<Event, HErr> {
    match push.tag {
        PushTag::User => {
            let umsg = serde_cbor::from_slice(&push.msg)?;
            handle_cmessage(push.timestamp, umsg).await
        }
        PushTag::Device => {
            let dmsg = serde_cbor::from_slice(&push.msg)?;
            handle_dmessage(push.timestamp, dmsg).await
        }
    }
}

async fn handle_cmessage(ts: Time, cm: ConversationMessage) -> Result<Event, HErr> {
    use ConversationMessageBody::*;
    let mut ev = Event::default();

    let cid = cm.cid();

    let msgs = cm.open().await?;

    for (msg, from) in msgs {
        match msg {
            NewKey(nk) => crate::contact_keys::add_keys(from.uid, &[nk.0])?,
            DepKey(dk) => crate::contact_keys::deprecate_keys(&[dk.0])?,
            AddedToConvo(ac) => {
                let mut db = crate::db::Database::get()?;
                {
                    let tx = db.transaction()?;

                    let cid = ac.cid;
                    let title = ac.title;

                    let mut conv_builder = crate::conversation::ConversationBuilder::new();
                    conv_builder.conversation_id(cid);

                    if let Some(title) = title {
                        conv_builder.title(title);
                    }

                    conv_builder.add_with_tx(&tx)?;
                    crate::members::db::add_members_with_tx(&tx, cid, &ac.members)?;
                    tx.commit()?;
                }

                cid.store_genesis(&ac.gen).await?;

                ev.notifications.push(Notification::NewConversation(cid));
            }
            ContactReqAck(cr) => ev
                .notifications
                .push(Notification::AddContactResponse(cid, from.uid, cr.0)),
            NewMembers(nm) => {
                let mut db = crate::db::Database::get()?;
                let tx = db.transaction()?;
                crate::members::db::add_members_with_tx(&tx, cid, &nm.0)?;
                tx.commit()?;
            }
            Msg(msg) => {
                let cmessages::Msg { mid, content, op } = msg;
                let cmessages::Message {
                    body,
                    attachments,
                    expiration,
                } = content;

                let mut builder = crate::message::InboundMessageBuilder::default();

                builder
                    .id(mid)
                    .author(from.uid)
                    .conversation_id(cid)
                    .attachments(attachments)
                    .timestamp(ts);

                if let Some(body) = body {
                    builder.body(body);
                }

                if let Some(op) = op {
                    builder.replying_to(op);
                }

                if let Some(expiration) = expiration {
                    builder.expiration(expiration);
                }

                builder.store()?;

                ev.notifications.push(Notification::NewMsg(mid, cid));
                ev.replies.push((cid, form_ack(mid)?));
            }
            Ack(ack) => {
                crate::message::add_receipt(ack.of, from.uid, ack.stat)?;
                ev.notifications
                    .push(Notification::MsgReceipt { mid: ack.of, cid });
            }
            Settings(update) => {
                update.apply(&cid)?;
                ev.notifications.push(Notification::Settings(cid, update));
            }
        }
    }

    Ok(ev)
}

async fn handle_dmessage(_: Time, msg: DeviceMessage) -> Result<Event, HErr> {
    let mut ev = Event::default();

    let (from, msg) = msg.open()?;

    match msg {
        DeviceMessageBody::ContactReq(cr) => {
            let dmessages::ContactReq { gen, cid } = cr;
            if gen.verify_sig(&from.did) {
                crate::contact::ContactBuilder::new(from.uid)
                    .pairwise_conversation(cid)
                    .add()?;

                cid.store_genesis(&gen).await?;

                ev.notifications
                    .push(Notification::NewContact(from.uid, cid));

                ev.replies.push((
                    cid,
                    ConversationMessageBody::ContactReqAck(cmessages::ContactReqAck(true)),
                ))
            }
        }
    }

    Ok(ev)
}
