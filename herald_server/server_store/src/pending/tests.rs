use super::*;
use crate::tests::get_client;
use crate::{w, wa};
use futures::stream::iter;
use protocol::auth::register::ServeEvent;
use serial_test_derive::serial;
use std::collections::BTreeSet as Set;
use std::convert::TryInto;
use womp::*;

impl PushedTo {
    fn is_missing(&self) -> bool {
        match self {
            PushedTo::Missing(_) => true,
            _ => false,
        }
    }

    fn no_recips(&self) -> bool {
        match self {
            PushedTo::NoRecipients => true,
            _ => false,
        }
    }
}

fn setup() -> (Push, UserId, sig::KeyPair) {
    let uid = w!("a".try_into());
    let kp = sig::KeyPair::gen_new();
    let did = *kp.public_key();
    let gid = GlobalId { uid, did };

    (
        Push {
            msg: Bytes::from_static(b"test"),
            timestamp: Time::now(),
            tag: PushTag::Key,
            gid,
        },
        uid,
        kp,
    )
}

fn same_devs(
    a: &[sig::PublicKey],
    b: &[sig::PublicKey],
) {
    let set = |keys: &[sig::PublicKey]| keys.iter().copied().collect::<Set<_>>();

    let sa = set(a);
    let sb = set(b);
    assert_eq!(sa.len(), sb.len());
    assert_eq!(sa, sb);
}

async fn check_pending(
    client: &mut Conn,
    push: &Push,
    devs: Vec<sig::PublicKey>,
) {
    for k in devs.into_iter().collect::<Set<_>>() {
        let pending = wa!(client.get_pending(k));
        assert_eq!(pending.len(), 1);

        let pending = pending
            .into_iter()
            .map(|(p, ix)| {
                assert_eq!(&p, push);
                ix
            })
            .collect::<Vec<_>>();

        wa!(client.del_pending(k, iter(pending)));

        let pending = wa!(client.get_pending(k));
        assert!(pending.is_empty());
    }
}

#[tokio::test]
#[serial]
async fn one_key() {
    let mut client = wa!(get_client());

    let (push, a_uid, a_kp) = setup();

    let a_kp2 = sig::KeyPair::gen_new();

    let a_init = a_kp.sign(a_uid);
    let a_endorse = sig::SigUpdate::Endorse(a_kp2.sign(a_uid));
    let a_add = a_kp.sign(a_endorse);

    let recip = Recip::One(SingleRecip::Key(*a_kp2.public_key()));

    assert!(wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)).is_missing());

    wa!(client.new_user(a_init));
    wa!(client.add_to_sigchain(a_add));

    let devs = match wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)) {
        PushedTo::PushedTo { devs, .. } => {
            assert_eq!(&devs, &[*a_kp2.public_key()]);
            devs
        }
        _ => panic!(),
    };

    check_pending(&mut client, &push, devs).await;
}

#[tokio::test]
#[serial]
async fn one_user() {
    let mut client = wa!(get_client());

    let (push, a_uid, a_kp) = setup();
    let a_init = a_kp.sign(a_uid);

    let b_uid: UserId = "b".try_into().expect(womp!());
    let b_kp = sig::KeyPair::gen_new();
    let b_init = b_kp.sign(b_uid);

    let recip = Recip::One(SingleRecip::User(b_uid));

    assert!(wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)).is_missing());

    wa!(client.new_user(a_init));
    wa!(client.new_user(b_init));

    let devs = match wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)) {
        PushedTo::PushedTo { devs, .. } => {
            assert_eq!(&devs, &[*b_kp.public_key()]);
            devs
        }
        _ => panic!(),
    };

    check_pending(&mut client, &push, devs).await;
}

#[tokio::test]
#[serial]
async fn many_keys() {
    let mut client = wa!(get_client());

    let (push, a_uid, a_kp) = setup();
    let a_init = a_kp.sign(a_uid);

    let a_kp2 = sig::KeyPair::gen_new();
    let a_endorse = sig::SigUpdate::Endorse(a_kp2.sign(a_uid));
    let a_add = a_kp.sign(a_endorse);

    let b_uid: UserId = "b".try_into().expect(womp!());
    let b_kp = sig::KeyPair::gen_new();
    let b_init = b_kp.sign(b_uid);

    let keys = vec![*a_kp2.public_key(), *b_kp.public_key()];
    let recip = Recip::Many(Recips::Keys(keys.clone()));

    assert!(wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)).is_missing());

    wa!(client.new_user(a_init));
    wa!(client.add_to_sigchain(a_add));
    wa!(client.new_user(b_init));

    let devs = match wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)) {
        PushedTo::PushedTo { devs, .. } => {
            same_devs(&devs, &keys);
            devs
        }
        _ => panic!(),
    };

    check_pending(&mut client, &push, devs).await;
}

#[tokio::test]
#[serial]
async fn many_users() {
    let mut client = wa!(get_client());

    let (push, a_uid, a_kp) = setup();
    let a_init = a_kp.sign(a_uid);

    let b_uid: UserId = "b".try_into().expect(womp!());
    let b_kp = sig::KeyPair::gen_new();
    let b_init = b_kp.sign(b_uid);

    let c_uid: UserId = "c".try_into().expect(womp!());
    let c_kp = sig::KeyPair::gen_new();
    let c_init = c_kp.sign(c_uid);

    let keys = vec![*b_kp.public_key(), *c_kp.public_key()]
        .into_iter()
        .collect::<Vec<_>>();
    let users = vec![b_uid, c_uid];
    let recip = Recip::Many(Recips::Users(users.clone()));

    assert!(wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)).is_missing());

    assert_eq!(wa!(client.new_user(a_init)), ServeEvent::Success);
    assert_eq!(wa!(client.new_user(b_init)), ServeEvent::Success);
    assert_eq!(
        client.new_user(c_init).await.expect(womp!()),
        ServeEvent::Success
    );

    let devs = match wa!(client.add_to_pending_and_get_valid_devs(&recip, &push)) {
        PushedTo::PushedTo { devs, .. } => {
            same_devs(&devs, &keys);
            devs
        }
        _ => panic!(),
    };

    check_pending(&mut client, &push, devs).await;
}
