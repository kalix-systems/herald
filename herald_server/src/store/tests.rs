use super::*;
use lazy_static::*;
use serial_test_derive::serial;
use std::convert::TryInto;
use womp::*;

lazy_static! {
    static ref POOL: Pool = init_pool();
}

fn open_conn() -> Conn {
    let conn = POOL.get().expect("Failed to get connection");
    diesel::delete(pending::table)
        .execute(conn.deref())
        .expect(womp!());
    diesel::delete(pushes::table)
        .execute(conn.deref())
        .expect(womp!());
    diesel::delete(prekeys::table)
        .execute(conn.deref())
        .expect(womp!());
    diesel::delete(userkeys::table)
        .execute(conn.deref())
        .expect(womp!());
    diesel::delete(keys::table)
        .execute(conn.deref())
        .expect(womp!());

    Conn(conn)
}

#[test]
#[serial]
fn device_exists() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().unwrap();

    let signed_pk = kp.sign(*kp.public_key());
    assert!(!conn.device_exists(kp.public_key()).unwrap());

    conn.register_user(user_id, signed_pk).unwrap();

    assert!(conn.device_exists(kp.public_key()).unwrap());
}

#[test]
#[serial]
fn register_and_add() {
    let mut conn = open_conn();

    let kp1 = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().unwrap();

    let signed_pk1 = kp1.sign(*kp1.public_key());
    assert!(!conn.device_exists(kp1.public_key()).unwrap());

    conn.register_user(user_id, signed_pk1).unwrap();

    assert!(conn.device_exists(kp1.public_key()).unwrap());

    let kp2 = sig::KeyPair::gen_new();
    let signed_pk2 = kp1.sign(*kp2.public_key());

    assert!(conn.device_exists(kp1.public_key()).unwrap());
    assert!(!conn.device_exists(kp2.public_key()).unwrap());

    conn.add_key(signed_pk2).unwrap();

    assert!(conn.device_exists(kp2.public_key()).unwrap());
}

#[test]
#[serial]
fn register_twice() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().unwrap();

    let signed_pk = kp.sign(*kp.public_key());

    conn.register_user(user_id, signed_pk).unwrap();

    let kp = sig::KeyPair::gen_new();
    let signed_pk = kp.sign(*kp.public_key());

    match conn.register_user(user_id, signed_pk) {
        Ok(register::Res::UIDTaken) => {}
        _ => panic!(),
    }
}

#[test]
#[serial]
fn read_key() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().unwrap();

    assert!(conn.read_key(*kp.public_key()).is_err());

    let signed_pk = kp.sign(*kp.public_key());

    conn.register_user(user_id, signed_pk).unwrap();

    assert!(conn.key_is_valid(*kp.public_key()).unwrap());

    let meta = conn
        .read_key(*kp.public_key())
        .expect("Couldn't read key meta");

    assert!(meta.key_is_valid(*kp.public_key()));

    conn.deprecate_key(signed_pk).unwrap();

    let meta = conn
        .read_key(*kp.public_key())
        .expect("Couldn't read key meta");

    assert!(!meta.key_is_valid(*kp.public_key()));
}

#[test]
#[serial]
fn user_exists() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().unwrap();

    let signed_pk = kp.sign(*kp.public_key());
    assert!(!conn.user_exists(&user_id).unwrap());

    conn.register_user(user_id, signed_pk).unwrap();

    assert!(conn.user_exists(&user_id).unwrap());
}

#[test]
#[serial]
fn read_meta() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().unwrap();

    let signed_pk = kp.sign(*kp.public_key());

    conn.register_user(user_id, signed_pk).unwrap();

    let keys = conn.read_meta(&user_id).unwrap().keys;
    assert_eq!(keys.len(), 1);
}

#[test]
#[serial]
fn valid_keys() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().unwrap();

    let signed_pk = kp.sign(*kp.public_key());

    conn.register_user(user_id, signed_pk).unwrap();

    let keys = conn.valid_keys(&user_id).unwrap();
    assert_eq!(keys.len(), 1);

    conn.deprecate_key(signed_pk).unwrap();
    let keys = conn.valid_keys(&user_id).unwrap();
    assert_eq!(keys.len(), 0);
}

#[test]
#[serial]
fn add_get_expire_pending() {
    for i in 0..10 {
        let mut conn = open_conn();

        let kp = sig::KeyPair::gen_new();
        let user_id = "Hello".try_into().unwrap();

        let signed_pk = kp.sign(*kp.public_key());
        conn.register_user(user_id, signed_pk).unwrap();

        let pending = conn.get_pending(*kp.public_key(), 1).unwrap();
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
                let mut conn = Conn(POOL.get().expect("failed to get connection"));
                assert!(conn.add_pending(vec![pk], [p].iter()).is_ok());
            })
        };

        let h1 = addf(push1.clone());
        let h2 = addf(push2.clone());
        let h3 = addf(push3.clone());

        h1.join().expect("first insert failed");
        h2.join().expect("second insert failed");
        h3.join().expect("third insert failed");

        let mut pushes = vec![push1, push2, push3];
        let pushes_unsorted = pushes.clone();
        pushes.sort_unstable_by_key(|p| p.timestamp);
        assert_eq!(pushes, pushes_unsorted);

        let pending = conn.get_pending(*kp.public_key(), 1).unwrap();
        assert_eq!(pending.as_slice(), &pushes[..1]);

        let pending = conn.get_pending(*kp.public_key(), 2).unwrap();
        assert_eq!(pending.as_slice(), &pushes[..2]);

        let pending = conn.get_pending(*kp.public_key(), 3).unwrap();
        assert_eq!(pending.as_slice(), &pushes[..3]);

        let pending = conn.get_pending(*kp.public_key(), 4).unwrap();
        assert_eq!(pending.as_slice(), &pushes[..3]);

        assert!(conn.expire_pending(*kp.public_key(), 1).is_ok());

        let pending = conn.get_pending(*kp.public_key(), 1).unwrap();
        assert_eq!(pending.as_slice(), &pushes[1..2]);

        let pending = conn.get_pending(*kp.public_key(), 2).unwrap();
        assert_eq!(pending.as_slice(), &pushes[1..3]);

        let pending = conn.get_pending(*kp.public_key(), 3).unwrap();
        assert_eq!(pending.as_slice(), &pushes[1..3]);

        assert!(conn.expire_pending(*kp.public_key(), 2).is_ok());

        let pending = conn.get_pending(*kp.public_key(), 1).unwrap();
        assert!(pending.is_empty());

        dbg!(i);
    }
}

#[test]
#[serial]
fn add_and_get_prekey() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let signed_pk = kp.sign(*kp.public_key());
    let user_id = "Hello".try_into().unwrap();
    conn.register_user(user_id, signed_pk).unwrap();

    let sealed_kp1 = sealed::KeyPair::gen_new();
    let sealed_kp2 = sealed::KeyPair::gen_new();

    let sealed_pk1 = sealed_kp1.sign_pub(&kp);
    let sealed_pk2 = sealed_kp2.sign_pub(&kp);

    conn.add_prekeys(&[sealed_pk1, sealed_pk2]).unwrap();

    let retrieved = conn.pop_prekeys(&[*kp.public_key()]).unwrap()[0].unwrap();
    assert!(retrieved == sealed_pk1 || retrieved == sealed_pk2);

    let retrieved = conn.pop_prekeys(&[*kp.public_key()]).unwrap()[0].unwrap();
    assert!(retrieved == sealed_pk1 || retrieved == sealed_pk2);

    assert!(conn.pop_prekeys(&[*kp.public_key()]).unwrap()[0].is_none());
}

#[test]
#[serial]
fn key_is_valid() {
    let mut conn = open_conn();

    let kp = sig::KeyPair::gen_new();
    let user_id = "Hello".try_into().unwrap();

    let signed_pk = kp.sign(*kp.public_key());
    assert!(!conn.key_is_valid(*kp.public_key()).unwrap());

    conn.register_user(user_id, signed_pk).unwrap();

    assert!(conn.key_is_valid(*kp.public_key()).unwrap());

    let signed_pk = kp.sign(*kp.public_key());

    conn.deprecate_key(signed_pk).unwrap();

    assert!(!conn.key_is_valid(*kp.public_key()).unwrap());
}

#[test]
#[serial]
fn double_deprecation() {
    let mut conn = open_conn();

    let kp1 = sig::KeyPair::gen_new();
    let user_id = "hello".try_into().unwrap();

    let signed_pk1 = kp1.sign(*kp1.public_key());

    conn.register_user(user_id, signed_pk1).unwrap();

    let kp2 = sig::KeyPair::gen_new();
    let signed_pk2 = kp1.sign(*kp2.public_key());

    conn.add_key(signed_pk2).unwrap();

    conn.deprecate_key(signed_pk2).unwrap();

    match conn.deprecate_key(signed_pk2) {
        Ok(PKIResponse::Redundant) => {}
        // ok(pkiresponse::deadkey) => {}
        _ => panic!(),
    }
}

#[test]
#[serial]
fn invalid_deprecation() {
    let mut conn = open_conn();

    let kp1 = sig::KeyPair::gen_new();
    let user_id = "hello".try_into().unwrap();

    let signed_pk1 = kp1.sign(*kp1.public_key());

    conn.register_user(user_id, signed_pk1).unwrap();

    let kp2 = sig::KeyPair::gen_new();
    let signed_pk2 = kp1.sign(*kp2.public_key());

    conn.add_key(signed_pk2).unwrap();

    conn.deprecate_key(signed_pk1).unwrap();

    match conn.deprecate_key(signed_pk2) {
        Ok(PKIResponse::DeadKey) => {}
        _ => panic!(),
    }
}
