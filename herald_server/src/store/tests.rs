use super::*;
use lazy_static::*;
use serial_test_derive::serial;
use std::convert::TryInto;
use tokio::runtime::current_thread::Runtime;
use womp::*;

//lazy_static! {
//    static ref POOL: Pool = init_pool();
//}

macro_rules! b {
    ($fut: expr) => {{
        let mut rt = Runtime::new().expect(womp!());

        rt.block_on($fut).expect(womp!())
    }};
}

macro_rules! c {
    ($fut: expr) => {{
        let mut rt = Runtime::new().expect(womp!());

        rt.block_on($fut)
    }};
}

fn open_conn() -> Conn {
    let mut rt = Runtime::new().expect(womp!());

    rt.block_on(async {
        let client = get_client().await.expect(womp!());

        // drop
        client
            .batch_execute(include_str!(
                "../../migrations/2019-09-21-221007_herald/down.sql"
            ))
            .await
            .expect(womp!());

        // create
        client
            .batch_execute(include_str!(
                "../../migrations/2019-09-21-221007_herald/up.sql"
            ))
            .await
            .expect(womp!());

        client
    })
}

#[test]
#[serial]
fn device_exists() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().expect(womp!());

    let signed_pk = kp.sign(*kp.public_key());
    assert!(!b!(conn.device_exists(kp.public_key())));

    b!(conn.register_user(user_id, signed_pk));

    assert!(b!(conn.device_exists(kp.public_key())));
}

#[test]
#[serial]
fn register_and_add() {
    let mut conn = open_conn();

    let kp1 = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().expect(womp!());

    let signed_pk1 = kp1.sign(*kp1.public_key());
    assert!(!b!(conn.device_exists(kp1.public_key())));

    b!(conn.register_user(user_id, signed_pk1));

    assert!(b!(conn.device_exists(kp1.public_key())));

    let kp2 = sig::KeyPair::gen_new();
    let signed_pk2 = kp1.sign(*kp2.public_key());

    assert!(b!(conn.device_exists(kp1.public_key())));

    assert!(!b!(conn.device_exists(kp2.public_key())));

    b!(conn.add_key(signed_pk2));

    assert!(b!(conn.device_exists(kp2.public_key())));
}

#[test]
#[serial]
fn register_twice() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().expect(womp!());

    let signed_pk = kp.sign(*kp.public_key());

    b!(conn.register_user(user_id, signed_pk));

    let kp = sig::KeyPair::gen_new();
    let signed_pk = kp.sign(*kp.public_key());

    match c!(conn.register_user(user_id, signed_pk)) {
        Ok(register::Res::UIDTaken) => {}
        _ => panic!(),
    }
}

#[test]
#[serial]
fn read_key() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().expect(womp!());

    assert!(c!(conn.read_key(*kp.public_key())).is_err());

    let signed_pk = kp.sign(*kp.public_key());

    b!(conn.register_user(user_id, signed_pk));

    assert!(b!(conn.key_is_valid(*kp.public_key())));

    let meta = b!(conn.read_key(*kp.public_key()));

    assert!(meta.key_is_valid(*kp.public_key()));

    b!(conn.deprecate_key(signed_pk));

    let meta = b!(conn.read_key(*kp.public_key()));

    assert!(!meta.key_is_valid(*kp.public_key()));
}

#[test]
#[serial]
fn user_exists() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().expect(womp!());

    let signed_pk = kp.sign(*kp.public_key());
    assert!(!b!(conn.user_exists(&user_id)));

    b!(conn.register_user(user_id, signed_pk));

    assert!(b!(conn.user_exists(&user_id)));
}

#[test]
#[serial]
fn read_meta() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().expect(womp!());

    let signed_pk = kp.sign(*kp.public_key());

    b!(conn.register_user(user_id, signed_pk));

    let keys = b!(conn.read_meta(&user_id)).keys;
    assert_eq!(keys.len(), 1);
}

#[test]
#[serial]
fn valid_keys() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().expect(womp!());

    let signed_pk = kp.sign(*kp.public_key());

    b!(conn.register_user(user_id, signed_pk));

    let keys = b!(conn.valid_keys(&user_id));
    assert_eq!(keys.len(), 1);

    b!(conn.deprecate_key(signed_pk));
    let keys = b!(conn.valid_keys(&user_id));
    assert_eq!(keys.len(), 0);
}

#[test]
#[serial]
fn add_get_expire_pending_ts() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().expect(womp!());

    let signed_pk = kp.sign(*kp.public_key());
    b!(conn.register_user(user_id, signed_pk));

    let pending = b!(conn.get_pending(*kp.public_key(), 1));

    assert_eq!(pending.len(), 0);

    let push1 = Push {
        tag: PushTag::User,
        timestamp: Time::now(),
        msg: bytes::Bytes::from_static(b"a"),
    };

    std::thread::sleep(std::time::Duration::from_secs(1));

    let push2 = Push {
        tag: PushTag::User,
        timestamp: Time::now(),
        msg: bytes::Bytes::from_static(b"b"),
    };

    std::thread::sleep(std::time::Duration::from_secs(1));

    let push3 = Push {
        tag: PushTag::User,
        timestamp: Time::now(),
        msg: bytes::Bytes::from_static(b"c"),
    };

    let addf = |p: Push| {
        let pk = *kp.public_key();
        std::thread::spawn(move || {
            let mut conn = b!(get_client());
            assert!(c!(conn.add_pending(vec![pk], [p].iter())).is_ok());
        })
    };

    let h1 = addf(push1.clone());
    let h2 = addf(push2.clone());
    let h3 = addf(push3.clone());

    h1.join().expect(womp!("first insert failed"));
    h2.join().expect(womp!("second insert failed"));
    h3.join().expect(womp!("third insert failed"));

    let mut pushes = vec![push1, push2, push3];
    let pushes_unsorted = pushes.clone();
    pushes.sort_unstable_by_key(|p| p.timestamp);
    assert_eq!(pushes, pushes_unsorted);

    let pending = b!(conn.get_pending(*kp.public_key(), 1));
    assert_eq!(pending.as_slice(), &pushes[..1]);

    let pending = b!(conn.get_pending(*kp.public_key(), 2));
    assert_eq!(pending.as_slice(), &pushes[..2]);

    dbg!();
    let pending = b!(conn.get_pending(*kp.public_key(), 3));
    dbg!();
    assert_eq!(pending.as_slice(), &pushes[..3]);
    dbg!();

    let pending = b!(conn.get_pending(*kp.public_key(), 4));
    dbg!();
    assert_eq!(pending.as_slice(), &pushes[..3]);
    dbg!();

    b!(conn.expire_pending(*kp.public_key(), 1));
    dbg!();

    let pending = b!(conn.get_pending(*kp.public_key(), 1));
    dbg!();
    assert_eq!(pending.as_slice(), &pushes[1..2]);
    dbg!();

    let pending = b!(conn.get_pending(*kp.public_key(), 2));
    dbg!();
    assert_eq!(pending.as_slice(), &pushes[1..3]);
    dbg!();

    let pending = b!(conn.get_pending(*kp.public_key(), 3));
    dbg!();
    assert_eq!(pending.as_slice(), &pushes[1..3]);
    dbg!();

    assert!(c!(conn.expire_pending(*kp.public_key(), 2)).is_ok());

    let pending = b!(conn.get_pending(*kp.public_key(), 1));
    assert!(pending.is_empty());
}
//
//#[test]
//#[serial]
//fn add_get_expire_pending_id() {
//    for i in 0..10 {
//        let mut conn = open_conn();
//
//        let kp = sig::KeyPair::gen_new();
//        let user_id = "Hello".try_into().expect(womp!());
//
//        let signed_pk = kp.sign(*kp.public_key());
//        conn.register_user(user_id, signed_pk).expect(womp!());
//
//        let pending = conn.get_pending(*kp.public_key(), 1).expect(womp!());
//        assert_eq!(pending.len(), 0);
//
//        let push1 = Push {
//            tag: PushTag::User,
//            timestamp: Time::now(),
//            msg: bytes::Bytes::from_static(b"a"),
//        };
//        let push2 = Push {
//            tag: PushTag::User,
//            timestamp: Time::now(),
//            msg: bytes::Bytes::from_static(b"b"),
//        };
//        let push3 = Push {
//            tag: PushTag::User,
//            timestamp: Time::now(),
//            msg: bytes::Bytes::from_static(b"c"),
//        };
//
//        let mut pushes = vec![push1, push2, push3];
//        let pushes_unsorted = pushes.clone();
//        pushes.sort_by_key(|p| p.timestamp);
//        assert_eq!(pushes, pushes_unsorted);
//
//        conn.add_pending(vec![*kp.public_key()], pushes.iter())
//            .expect("failed to add pushes");
//
//        let pending = conn.get_pending(*kp.public_key(), 1).expect(womp!());
//        assert_eq!(pending.as_slice(), &pushes[..1]);
//
//        let pending = conn.get_pending(*kp.public_key(), 2).expect(womp!());
//        assert_eq!(pending.as_slice(), &pushes[..2]);
//
//        let pending = conn.get_pending(*kp.public_key(), 3).expect(womp!());
//        assert_eq!(pending.as_slice(), &pushes[..3]);
//
//        let pending = conn.get_pending(*kp.public_key(), 4).expect(womp!());
//        assert_eq!(pending.as_slice(), &pushes[..3]);
//
//        assert!(conn.expire_pending(*kp.public_key(), 1).is_ok());
//
//        let pending = conn.get_pending(*kp.public_key(), 1).expect(womp!());
//        assert_eq!(pending.as_slice(), &pushes[1..2]);
//
//        let pending = conn.get_pending(*kp.public_key(), 2).expect(womp!());
//        assert_eq!(pending.as_slice(), &pushes[1..3]);
//
//        let pending = conn.get_pending(*kp.public_key(), 3).expect(womp!());
//        assert_eq!(pending.as_slice(), &pushes[1..3]);
//
//        assert!(conn.expire_pending(*kp.public_key(), 2).is_ok());
//
//        let pending = conn.get_pending(*kp.public_key(), 1).expect(womp!());
//        assert!(pending.is_empty());
//
//        dbg!(i);
//    }
//}
//#[test]
//#[serial]
//fn add_and_get_prekey() {
//    let mut conn = open_conn();
//
//    let kp = sig::KeyPair::gen_new();
//    let signed_pk = kp.sign(*kp.public_key());
//    let user_id = "Hello".try_into().expect(womp!());
//    conn.register_user(user_id, signed_pk).expect(womp!());
//
//    let sealed_kp1 = sealed::KeyPair::gen_new();
//    let sealed_kp2 = sealed::KeyPair::gen_new();
//
//    let sealed_pk1 = sealed_kp1.sign_pub(&kp);
//    let sealed_pk2 = sealed_kp2.sign_pub(&kp);
//
//    conn.add_prekeys(&[sealed_pk1, sealed_pk2]).expect(womp!());
//
//    let retrieved = conn.pop_prekeys(&[*kp.public_key()]).expect(womp!())[0].expect(womp!());
//    assert!(retrieved == sealed_pk1 || retrieved == sealed_pk2);
//
//    let retrieved = conn.pop_prekeys(&[*kp.public_key()]).expect(womp!())[0].expect(womp!());
//    assert!(retrieved == sealed_pk1 || retrieved == sealed_pk2);
//
//    assert!(conn.pop_prekeys(&[*kp.public_key()]).expect(womp!())[0].is_none());
//}
//
//#[test]
//#[serial]
//fn key_is_valid() {
//    let mut conn = open_conn();
//
//    let kp = sig::KeyPair::gen_new();
//    let user_id = "Hello".try_into().expect(womp!());
//
//    let signed_pk = kp.sign(*kp.public_key());
//    assert!(!conn.key_is_valid(*kp.public_key()).expect(womp!()));
//
//    conn.register_user(user_id, signed_pk).expect(womp!());
//
//    assert!(conn.key_is_valid(*kp.public_key()).expect(womp!()));
//
//    let signed_pk = kp.sign(*kp.public_key());
//
//    conn.deprecate_key(signed_pk).expect(womp!());
//
//    assert!(!conn.key_is_valid(*kp.public_key()).expect(womp!()));
//}
//
//#[test]
//#[serial]
//fn double_deprecation() {
//    let mut conn = open_conn();
//
//    let kp1 = sig::KeyPair::gen_new();
//    let user_id = "hello".try_into().expect(womp!());
//
//    let signed_pk1 = kp1.sign(*kp1.public_key());
//
//    conn.register_user(user_id, signed_pk1).expect(womp!());
//
//    let kp2 = sig::KeyPair::gen_new();
//    let signed_pk2 = kp1.sign(*kp2.public_key());
//
//    conn.add_key(signed_pk2).expect(womp!());
//
//    conn.deprecate_key(signed_pk2).expect(womp!());
//
//    match conn.deprecate_key(signed_pk2) {
//        Ok(PKIResponse::Redundant) => {}
//        // ok(pkiresponse::deadkey) => {}
//        _ => panic!(),
//    }
//}
//
//#[test]
//#[serial]
//fn invalid_deprecation() {
//    let mut conn = open_conn();
//
//    let kp1 = sig::KeyPair::gen_new();
//    let user_id = "hello".try_into().expect(womp!());
//
//    let signed_pk1 = kp1.sign(*kp1.public_key());
//
//    conn.register_user(user_id, signed_pk1).expect(womp!());
//
//    let kp2 = sig::KeyPair::gen_new();
//    let signed_pk2 = kp1.sign(*kp2.public_key());
//
//    conn.add_key(signed_pk2).expect(womp!());
//
//    conn.deprecate_key(signed_pk1).expect(womp!());
//
//    match conn.deprecate_key(signed_pk2) {
//        Ok(PKIResponse::DeadKey) => {}
//        _ => panic!(),
//    }
//}
