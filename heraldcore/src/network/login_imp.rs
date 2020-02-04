use super::*;
use coremacros::w;

/// Attempts to login to the server, spawning a long-lived thread to handle messages pushed from
/// the server.
pub fn login() -> Result<(), HErr> {
    use login::*;

    kcl::init();

    CAUGHT_UP.store(false, Ordering::Release);

    let uid = w!(config::id());
    let kp = w!(config::keypair());

    let gid = GlobalId {
        uid,
        did: *kp.public(),
    };

    let wsurl = format!("ws://{}/login", home_server());

    let mut ws = w!(wsclient::ClientBuilder::new(&wsurl)
        .expect("failed to parse server url")
        .connect_insecure());

    w!(sock_send_msg(&mut ws, &SignAs(gid)));

    match w!(sock_get_msg(&mut ws)) {
        SignAsResponse::Sign(u) => {
            let token = LoginToken(sign_ser(&kp, u.as_ref()));
            w!(sock_send_msg(&mut ws, &token));

            match w!(sock_get_msg(&mut ws)) {
                LoginTokenResponse::Success => {}
                e => return Err(SignInFailed(e)),
            }
        }
        e => return Err(GIDSpecFailed(e)),
    }

    let ev = w!(catchup(&mut ws));

    CAUGHT_UP.store(true, Ordering::Release);

    // clear pending
    for (tag, cid, content) in w!(pending::get_pending()) {
        w!(send_cmessage(cid, &content));
        w!(pending::remove_pending(tag));
    }

    // send read receipts, etc
    w!(ev.execute());

    std::thread::spawn(move || {
        move || -> Result<(), HErr> {
            loop {
                w!(w!(catchup(&mut ws)).execute());
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
        let maybe_len = w!(sock_get_block::<_, u64>(ws));
        w!(sock_send_msg(ws, &maybe_len));
        match w!(sock_get_block(ws)) {
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
            packets.push(w!(sock_get_block(ws)));
        }
        match Packet::collect(&packets) {
            Some(v) => {
                // TODO: consider doing this later?
                // or maybe having a callback that has to succeeed here?
                // after the server receives this, it *will* delete the message,
                // so I'm inclined to be damn sure we're done with it
                w!(sock_send_msg(ws, &PacketResponse::Success));
                return Ok(w!(kson::from_bytes(v.into())));
            }
            None => {
                w!(sock_send_msg(ws, &PacketResponse::Retry));
            }
        }
    }
}

fn sock_get_block<S: websocket::stream::Stream, T: De>(
    ws: &mut wsclient::Client<S>
) -> Result<T, HErr> {
    loop {
        if let WMessage::Binary(v) = w!(ws.recv_message()) {
            return Ok(w!(kson::from_bytes(v.into())));
        }
    }
}

fn sock_send_msg<S: websocket::stream::Stream, T: Ser>(
    ws: &mut wsclient::Client<S>,
    t: &T,
) -> Result<(), HErr> {
    let m = WMessage::Binary(kson::to_vec(t));
    w!(ws.send_message(&m));
    Ok(())
}

fn handle_push(push: &Push) -> Result<Event, HErr> {
    match push.tag {
        PushTag::User => {
            let umsg = w!(kson::from_bytes(push.msg.clone()));
            handle_cmessage(push.timestamp, umsg)
        }
        PushTag::Device => {
            let dmsg = w!(kson::from_bytes(push.msg.clone()));
            handle_dmessage(push.timestamp, dmsg)
        }
    }
}

fn catchup<S: websocket::stream::Stream>(ws: &mut wsclient::Client<S>) -> Result<Event, HErr> {
    use catchup::*;

    let mut ev = Event::default();

    while let Catchup::Messages(p) = w!(sock_get_msg(ws)) {
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
        w!(sock_send_msg(ws, &CatchupAck(len)));
    }

    Ok(ev)
}
