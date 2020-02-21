use super::*;
use crate::tests::get_client;
use futures::stream::iter;
use protocol::auth::RegisterResponse;
use serial_test_derive::serial;
use sig::sign_ser as sign;
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
}

fn setup() -> (Push, UserId, sig::KeyPair) {
    let uid = "a".try_into().unwrap();
    let kp = sig::KeyPair::gen_new();
    let did = *kp.public();
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
        let pending = client.get_pending(k).await.unwrap();
        assert_eq!(pending.len(), 1);

        let pending = pending
            .into_iter()
            .map(|(p, ix)| {
                assert_eq!(&p, push);
                ix
            })
            .collect::<Vec<_>>();

        client.del_pending(k, iter(pending)).await.unwrap();

        let pending = client.get_pending(k).await.unwrap();
        assert!(pending.is_empty());
    }
}

#[tokio::test]
#[serial]
async fn one_key() {
    let mut client = get_client().await.unwrap();

    let (push, a_uid, a_kp) = setup();

    let a_kp2 = sig::KeyPair::gen_new();
    let a_init = sign(&a_kp, a_uid);
    let a_endorse = sig::SigUpdate::Endorse(sign(&a_kp2, a_uid));
    let a_add = sign(&a_kp, a_endorse);

    let recip = Recip::One(SingleRecip::Key(*a_kp2.public()));

    assert!(client
        .add_to_pending_and_get_valid_devs(&[(&recip, &push)])
        .await
        .unwrap()
        .recv()
        .await
        .unwrap()
        .0
        .is_missing());

    client.new_user(a_init).await.unwrap();
    client.add_to_sigchain(a_add).await.unwrap();

    let devs = match client
        .add_to_pending_and_get_valid_devs(&[(&recip, &push)])
        .await
        .unwrap()
        .recv()
        .await
        .unwrap()
        .0
    {
        PushedTo::PushedTo { devs, .. } => {
            assert_eq!(&devs, &[*a_kp2.public()]);
            devs
        }
        _ => panic!(),
    };

    check_pending(&mut client, &push, devs).await;
}

#[tokio::test]
#[serial]
async fn one_user() {
    let mut client = get_client().await.unwrap();

    let (push, a_uid, a_kp) = setup();
    let a_init = sign(&a_kp, a_uid);

    let b_uid: UserId = "b".try_into().expect(womp!());
    let b_kp = sig::KeyPair::gen_new();
    let b_init = sign(&b_kp, b_uid);

    let recip = Recip::One(SingleRecip::User(b_uid));

    assert!(client
        .add_to_pending_and_get_valid_devs(&[(&recip, &push)])
        .await
        .unwrap()
        .recv()
        .await
        .unwrap()
        .0
        .is_missing());

    client.new_user(a_init).await.unwrap();
    client.new_user(b_init).await.unwrap();

    let devs = match client
        .add_to_pending_and_get_valid_devs(&[(&recip, &push)])
        .await
        .unwrap()
        .recv()
        .await
        .unwrap()
        .0
    {
        PushedTo::PushedTo { devs, .. } => {
            assert_eq!(&devs, &[*b_kp.public()]);
            devs
        }
        _ => panic!(),
    };

    check_pending(&mut client, &push, devs).await;
}

#[tokio::test]
#[serial]
async fn many_keys() {
    let mut client = get_client().await.unwrap();

    let (push, a_uid, a_kp) = setup();
    let a_init = sign(&a_kp, a_uid);

    let a_kp2 = sig::KeyPair::gen_new();
    let a_endorse = sig::SigUpdate::Endorse(sign(&a_kp2, a_uid));
    let a_add = sign(&a_kp, a_endorse);

    let b_uid: UserId = "b".try_into().expect(womp!());
    let b_kp = sig::KeyPair::gen_new();
    let b_init = sign(&b_kp, b_uid);

    let keys = vec![*a_kp2.public(), *b_kp.public()];
    let recip = Recip::Many(Recips::Keys(keys.clone()));

    assert!(client
        .add_to_pending_and_get_valid_devs(&[(&recip, &push)])
        .await
        .unwrap()
        .recv()
        .await
        .unwrap()
        .0
        .is_missing());

    client.new_user(a_init).await.unwrap();
    client.add_to_sigchain(a_add).await.unwrap();
    client.new_user(b_init).await.unwrap();

    let devs = match client
        .add_to_pending_and_get_valid_devs(&[(&recip, &push)])
        .await
        .unwrap()
        .recv()
        .await
        .unwrap()
        .0
    {
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
    let mut client = get_client().await.unwrap();

    let (push, a_uid, a_kp) = setup();
    let a_init = sign(&a_kp, a_uid);

    let b_uid: UserId = "b".try_into().expect(womp!());
    let b_kp = sig::KeyPair::gen_new();
    let b_init = sign(&b_kp, b_uid);

    let c_uid: UserId = "c".try_into().expect(womp!());
    let c_kp = sig::KeyPair::gen_new();
    let c_init = sign(&c_kp, c_uid);

    let keys = vec![*b_kp.public(), *c_kp.public()]
        .into_iter()
        .collect::<Vec<_>>();
    let users = vec![b_uid, c_uid];
    let recip = Recip::Many(Recips::Users(users.clone()));

    assert!(client
        .add_to_pending_and_get_valid_devs(&[(&recip, &push)])
        .await
        .unwrap()
        .recv()
        .await
        .unwrap()
        .0
        .is_missing());

    assert_eq!(
        client.new_user(a_init).await.unwrap(),
        RegisterResponse::Success
    );
    assert_eq!(
        client.new_user(b_init).await.unwrap(),
        RegisterResponse::Success
    );
    assert_eq!(
        client.new_user(c_init).await.expect(womp!()),
        RegisterResponse::Success
    );

    let devs = match client
        .add_to_pending_and_get_valid_devs(&[(&recip, &push)])
        .await
        .unwrap()
        .recv()
        .await
        .unwrap()
        .0
    {
        PushedTo::PushedTo { devs, .. } => {
            same_devs(&devs, &keys);
            devs
        }
        _ => panic!(),
    };

    check_pending(&mut client, &push, devs).await;
}
