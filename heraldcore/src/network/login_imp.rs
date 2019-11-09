use super::*;

/// Attempts to login to the server, spawning a long-lived thread to handle messages pushed from
/// the server.
///
/// Takes a callback as an argument that is called whenever a message is received.
pub fn login<F, G>(mut f: F, mut g: G) -> Result<(), HErr>
where
    F: FnMut(Notification) + Send + 'static,
    G: FnMut(HErr) + Send + 'static,
{
    use login::*;

    if sodiumoxide::init().is_err() {
        eprintln!("failed to init libsodium - what are you doing");
        std::process::abort()
    }

    CAUGHT_UP.store(false, Ordering::Release);

    let uid = Config::static_id()?;
    let kp = Config::static_keypair()?;
    let gid = GlobalId {
        uid,
        did: *kp.public_key(),
    };

    let wsurl = format!("ws://{}/login", *SERVER_ADDR);
    let mut ws = wsclient::ClientBuilder::new(&wsurl)
        .expect("failed to parse server url")
        .connect_insecure()?;

    sock_send_msg(&mut ws, &SignAs(gid))?;

    match sock_get_msg(&mut ws)? {
        SignAsResponse::Sign(u) => {
            let token = LoginToken(kp.raw_sign_detached(u.as_ref()));
            sock_send_msg(&mut ws, &token)?;

            match sock_get_msg(&mut ws)? {
                LoginTokenResponse::Success => {}
                e => return Err(SignInFailed(e)),
            }
        }
        e => return Err(GIDSpecFailed(e)),
    }

    let ev = catchup(&mut ws)?;

    CAUGHT_UP.store(true, Ordering::Release);

    // clear pending
    for (tag, cid, content) in pending::get_pending()? {
        send_cmessage(cid, &content)?;
        pending::remove_pending(tag)?;
    }

    // send read receipts, etc
    ev.execute(&mut f, &mut g)?;

    std::thread::spawn(move || {
        move || -> Result<(), HErr> {
            loop {
                catchup(&mut ws)?.execute(&mut f, &mut g)?;
            }
        }()
        .unwrap_or_else(|e| eprintln!("login connection closed with message: {}", e));
        CAUGHT_UP.store(false, Ordering::Release);
    });

    Ok(())
}

fn catchup<S: websocket::stream::Stream>(ws: &mut wsclient::Client<S>) -> Result<Event, HErr> {
    use catchup::*;

    let mut ev = Event::default();

    while let Catchup::Messages(p) = sock_get_msg(ws)? {
        let len = p.len() as u64;
        for push in p.iter() {
            match handle_push(push) {
                Ok(e2) => ev.merge(e2),
                Err(e) => {
                    eprintln!("error while catching up, error was:\n{}", e);
                    ev.errors.push(e);
                }
            }
        }
        sock_send_msg(ws, &CatchupAck(len))?;
    }

    Ok(ev)
}

fn sock_get_msg<S: websocket::stream::Stream, T: for<'a> Deserialize<'a>>(
    ws: &mut wsclient::Client<S>,
) -> Result<T, HErr> {
    let len;

    loop {
        let maybe_len = sock_get_block(ws)?;
        sock_send_msg(ws, &maybe_len)?;
        match sock_get_block(ws)? {
            PacketResponse::Success => {
                len = maybe_len;
                break;
            }
            PacketResponse::Retry => {}
        }
    }

    loop {
        let mut packets = Vec::with_capacity(len as usize);
        for _ in 0..len {
            packets.push(sock_get_block(ws)?);
        }
        match Packet::collect(&packets) {
            Some(v) => {
                // TODO: consider doing this later?
                // or maybe having a callback that has to succeeed here?
                // after the server receives this, it *will* delete the message,
                // so I'm inclined to be damn sure we're done with it
                sock_send_msg(ws, &PacketResponse::Success)?;
                return Ok(serde_cbor::from_slice(&v)?);
            }
            None => {
                sock_send_msg(ws, &PacketResponse::Retry)?;
            }
        }
    }
}

fn sock_get_block<S: websocket::stream::Stream, T: for<'a> Deserialize<'a>>(
    ws: &mut wsclient::Client<S>,
) -> Result<T, HErr> {
    loop {
        if let WMessage::Binary(v) = ws.recv_message()? {
            return Ok(serde_cbor::from_slice(&v)?);
        }
    }
}

fn sock_send_msg<S: websocket::stream::Stream, T: Serialize>(
    ws: &mut wsclient::Client<S>,
    t: &T,
) -> Result<(), HErr> {
    let m = WMessage::Binary(serde_cbor::to_vec(t)?);
    ws.send_message(&m)?;
    Ok(())
}

fn handle_push(push: &Push) -> Result<Event, HErr> {
    match push.tag {
        PushTag::User => {
            let umsg = serde_cbor::from_slice(&push.msg)?;
            handle_cmessage(push.timestamp, umsg)
        }
        PushTag::Device => {
            let dmsg = serde_cbor::from_slice(&push.msg)?;
            handle_dmessage(push.timestamp, dmsg)
        }
    }
}

fn handle_cmessage(ts: Time, cm: ConversationMessage) -> Result<Event, HErr> {
    use ConversationMessageBody::*;
    let mut ev = Event::default();

    let cid = cm.cid();

    let msgs = cm.open()?;

    for (msg, from) in msgs {
        match msg {
            NewKey(nk) => crate::contact_keys::add_keys(from.uid, &[nk.0])?,
            DepKey(dk) => crate::contact_keys::deprecate_keys(&[dk.0])?,
            AddedToConvo(ac) => {
                let mut db = crate::db::Database::get()?;
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

                cid.store_genesis(&ac.gen)?;

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

fn handle_dmessage(_: Time, msg: DeviceMessage) -> Result<Event, HErr> {
    let mut ev = Event::default();

    let (from, msg) = msg.open()?;

    match msg {
        DeviceMessageBody::ContactReq(cr) => {
            let dmessages::ContactReq { gen, cid } = cr;
            if gen.verify_sig(&from.did) {
                crate::contact::ContactBuilder::new(from.uid)
                    .pairwise_conversation(cid)
                    .add()?;

                cid.store_genesis(&gen)?;

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
