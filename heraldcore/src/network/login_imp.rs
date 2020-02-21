use super::*;
use coremacros::w;
use herald_common::protocol::auth::*;

macro_rules! send {
    ($ws:expr, $send:expr) => {
        w!(sock_send_msg(&mut $ws, &$send));
    };
}

macro_rules! recv {
    ($ws:expr) => {
        w!(sock_get_msg(&mut $ws))
    };
    ($ws:expr, $tt:ty) => {
        w!(sock_get_msg::<_, $tt>(&mut $ws))
    };
}

/// Attempts to login to the server, spawning a long-lived thread to handle messages pushed from
/// the server.
pub fn login() -> Result<(), HErr> {
    use login_types::*;

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

    send!(ws, LOGIN);
    send!(ws, *kp.public());

    // for now assume the happy path
    {
        let res = recv!(ws, ClaimResponse);
        if res != ClaimResponse::Challenge {
            return Err(HeraldError(
                "login failed - did you register this device yet?".into(),
            ));
        }
    }

    // challenge phase
    let challenge = recv!(ws, UQ);
    let sig = kp.secret().sign(challenge.as_ref());
    send!(ws, sig);
    {
        let res = recv!(ws, ChallengeResult);
        if res != ChallengeResult::Success {
            return Err(HeraldError("login failed - bad sig".into()));
        }
    }

    let ev = w!(catchup(&mut ws));

    CAUGHT_UP.store(true, Ordering::Release);

    // clear pending
    for (tag, cid, content) in w!(pending::get_pending()) {
        w!(send_cmessage(cid, content));
        w!(pending::remove_pending(tag));
    }

    // send read receipts, etc
    w!(ev.execute());

    std::thread::spawn(move || {
        move || -> Result<(), HErr> {
            loop {
                let push = recv!(ws, Push);
                let ev = w!(handle_push(push));
                send!(ws, PushAck::Success);

                w!(ev.execute());
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

fn catchup<S: websocket::stream::Stream>(ws: &mut wsclient::Client<S>) -> Result<Event, HErr> {
    use catchup::*;

    let mut ev = Event::default();
    while Catchup::NewMessages == w!(sock_get_msg(ws)) {
        let pushes: Vec<Push> = w!(sock_get_msg(ws));
        let mut ev = Event::default();
        for push in pushes {
            let ev_ = match handle_push(push) {
                Ok(ev) => ev,
                Err(e) => {
                    w!(sock_send_msg(ws, &CatchupAck::Failure));
                    return Err(e.into());
                }
            };
            ev.merge(ev_);
        }
        w!(sock_send_msg(ws, &CatchupAck::Success));
    }

    Ok(ev)
}
