use super::*;

/// Attempts to login to the server, spawning a long-lived thread to handle messages pushed from
/// the server.
///
/// Takes a callback as an argument that is called whenever a message is received.
pub fn login<F, G>(
    mut f: F,
    mut g: G,
) -> Result<(), HErr>
where
    F: FnMut(Notification) + Send + 'static,
    G: FnMut(HErr) + Send + 'static,
{
    use login::*;

    CAUGHT_UP.store(false, Ordering::Release);

    let uid = config::id()?;
    let kp = config::keypair()?;

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
            let token = LoginToken(kp.secret_key().sign(u.as_ref()));
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

fn sock_get_msg<S: websocket::stream::Stream, T: De>(
    ws: &mut wsclient::Client<S>
) -> Result<T, HErr> {
    let len;

    loop {
        let maybe_len = sock_get_block::<_, u64>(ws)?;
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
                return Ok(kson::from_bytes(v.into())?);
            }
            None => {
                sock_send_msg(ws, &PacketResponse::Retry)?;
            }
        }
    }
}

fn sock_get_block<S: websocket::stream::Stream, T: De>(
    ws: &mut wsclient::Client<S>
) -> Result<T, HErr> {
    loop {
        if let WMessage::Binary(v) = ws.recv_message()? {
            return Ok(kson::from_bytes(v.into())?);
        }
    }
}

fn sock_send_msg<S: websocket::stream::Stream, T: Ser>(
    ws: &mut wsclient::Client<S>,
    t: &T,
) -> Result<(), HErr> {
    let m = WMessage::Binary(kson::to_vec(t));
    ws.send_message(&m)?;
    Ok(())
}

fn handle_push(push: &Push) -> Result<Event, HErr> {
    match push.tag {
        PushTag::User => {
            let umsg = kson::from_bytes(push.msg.clone())?;
            handle_cmessage(push.timestamp, umsg)
        }
        PushTag::Device => {
            let dmsg = kson::from_bytes(push.msg.clone())?;
            handle_dmessage(push.timestamp, dmsg)
        }
    }
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
