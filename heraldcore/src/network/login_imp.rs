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
            dbg!(res);
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
            dbg!(res);
            return Err(HeraldError("login failed - bad sig".into()));
        }
    }

    // let ev = w!(catchup(&mut ws));

    CAUGHT_UP.store(true, Ordering::Release);

    // // clear pending
    // for (tag, cid, content) in w!(pending::get_pending()) {
    //     w!(send_cmessage(cid, &content));
    //     w!(pending::remove_pending(tag));
    // }

    // // send read receipts, etc
    // w!(ev.execute());

    // std::thread::spawn(move || {
    //     move || -> Result<(), HErr> {
    //         loop {
    //             w!(w!(catchup(&mut ws)).execute());
    //         }
    //     }()
    //     .unwrap_or_else(|e| eprintln!("login connection closed with message: {}", e));
    //     CAUGHT_UP.store(false, Ordering::Release);
    // });

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

// fn handle_push(push: &Push) -> Result<Event, HErr> {
//     match push.tag {
//         PushTag::User => {
//             let umsg = w!(kson::from_bytes(push.msg.clone()));
//             handle_cmessage(push.timestamp, umsg)
//         }
//         PushTag::Device => {
//             let dmsg = w!(kson::from_bytes(push.msg.clone()));
//             handle_dmessage(push.timestamp, dmsg)
//         }
//     }
// }

// fn catchup<S: websocket::stream::Stream>(ws: &mut wsclient::Client<S>) -> Result<Event, HErr> {
//     use catchup::*;

//     let mut ev = Event::default();

//     while let Catchup::Messages(p) = w!(sock_get_msg(ws)) {
//         let len = p.len() as u64;
//         for push in p.iter() {
//             match handle_push(push) {
//                 Ok(e2) => ev.merge(e2),
//                 Err(e) => {
//                     eprintln!("error while catching up, error was:\n{}", e);
//                     ev.errors.push(e);
//                 }
//             }
//         }
//         w!(sock_send_msg(ws, &CatchupAck(len)));
//     }

//     Ok(ev)
// }
