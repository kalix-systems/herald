//use super::*;
//use serial_test_derive::serial;
//use std::convert::TryInto;
//use womp::*;
//
//macro_rules! w {
//    ($maybe_val: expr) => {
//        $maybe_val.expect(womp!())
//    };
//}
//
//macro_rules! wa {
//    ($maybe_fut: expr) => {
//        w!($maybe_fut.await)
//    };
//}
//
//async fn get_client() -> Result<Conn, Error> {
//    let pool = Pool::new();
//    pool.get().await
//}

//#[tokio::test]
//#[serial]
//async fn device_exists() {
//    let mut client = wa!(get_client());
//    wa!(client.reset_all());
//
//    let kp = sig::KeyPair::gen_new();
//    let user_id = "Hello".try_into().expect(womp!());
//
//    let signed_pk = kp.sign(*kp.public_key());
//    assert!(!wa!(client.device_exists(kp.public_key())));
//
//    wa!(client.register_user(user_id, signed_pk));
//
//    assert!(wa!(client.device_exists(kp.public_key())));
//}
//
//#[tokio::test]
//#[serial]
//async fn register_and_add() {
//    let mut client = wa!(get_client());
//    wa!(client.reset_all());
//
//    let kp1 = sig::KeyPair::gen_new();
//    let user_id = "Hello".try_into().expect(womp!());
//
//    let signed_pk1 = kp1.sign(*kp1.public_key());
//    assert!(!wa!(client.device_exists(kp1.public_key())));
//
//    wa!(client.register_user(user_id, signed_pk1));
//
//    assert!(wa!(client.device_exists(kp1.public_key())));
//
//    let kp2 = sig::KeyPair::gen_new();
//    let signed_pk2 = kp1.sign(*kp2.public_key());
//
//    assert!(wa!(client.device_exists(kp1.public_key())));
//
//    assert!(!wa!(client.device_exists(kp2.public_key())));
//
//    wa!(client.add_key(signed_pk2));
//
//    assert!(wa!(client.device_exists(kp2.public_key())));
//}
//
//#[tokio::test]
//#[serial]
//async fn register_twice() {
//    let mut client = wa!(get_client());
//    wa!(client.reset_all());
//
//    let kp = sig::KeyPair::gen_new();
//    let user_id = "Hello".try_into().expect(womp!());
//
//    let signed_pk = kp.sign(*kp.public_key());
//
//    wa!(client.register_user(user_id, signed_pk));
//
//    let kp = sig::KeyPair::gen_new();
//    let signed_pk = kp.sign(*kp.public_key());
//
//    match client.register_user(user_id, signed_pk).await {
//        Ok(register::Res::UIDTaken) => {}
//        _ => panic!(),
//    }
//}
//
//#[tokio::test]
//#[serial]
//async fn read_key() {
//    let mut client = wa!(get_client());
//    wa!(client.reset_all());
//
//    let kp = sig::KeyPair::gen_new();
//    let user_id = "Hello".try_into().expect(womp!());
//
//    assert!(client.read_key(*kp.public_key()).await.is_err());
//
//    let signed_pk = kp.sign(*kp.public_key());
//
//    wa!(client.register_user(user_id, signed_pk));
//
//    assert!(wa!(client.key_is_valid(*kp.public_key())));
//
//    let meta = wa!(client.read_key(*kp.public_key()));
//
//    assert!(meta.key_is_valid(*kp.public_key()));
//
//    wa!(client.deprecate_key(signed_pk));
//
//    let meta = wa!(client.read_key(*kp.public_key()));
//
//    assert!(!meta.key_is_valid(*kp.public_key()));
//}
//
//#[tokio::test]
//#[serial]
//async fn user_exists() {
//    let mut client = wa!(get_client());
//    wa!(client.reset_all());
//
//    let kp = sig::KeyPair::gen_new();
//    let user_id = "Hello".try_into().expect(womp!());
//
//    let signed_pk = kp.sign(*kp.public_key());
//    assert!(!wa!(client.user_exists(&user_id)));
//
//    wa!(client.register_user(user_id, signed_pk));
//
//    assert!(wa!(client.user_exists(&user_id)));
//}
//
//#[tokio::test]
//#[serial]
//async fn read_meta() {
//    let mut client = wa!(get_client());
//    wa!(client.reset_all());
//
//    let kp = sig::KeyPair::gen_new();
//    let user_id = "Hello".try_into().expect(womp!());
//
//    let signed_pk = kp.sign(*kp.public_key());
//
//    wa!(client.register_user(user_id, signed_pk));
//
//    let keys = wa!(client.read_meta(&user_id)).keys;
//    assert_eq!(keys.len(), 1);
//}
//
//#[tokio::test]
//#[serial]
//async fn valid_keys() {
//    let mut client = wa!(get_client());
//    wa!(client.reset_all());
//
//    let kp = sig::KeyPair::gen_new();
//    let user_id = "Hello".try_into().expect(womp!());
//
//    let signed_pk = kp.sign(*kp.public_key());
//
//    wa!(client.register_user(user_id, signed_pk));
//
//    let keys = wa!(client.valid_keys(&user_id));
//    assert_eq!(keys.len(), 1);
//
//    wa!(client.deprecate_key(signed_pk));
//    let keys = wa!(client.valid_keys(&user_id));
//    assert_eq!(keys.len(), 0);
//}
//
//#[tokio::test]
//#[serial]
//async fn add_get_expire_pending_ts() {
//    let mut client = wa!(get_client());
//    wa!(client.reset_all());
//
//    let kp = sig::KeyPair::gen_new();
//    let user_id = "Hello".try_into().expect(womp!());
//
//    let signed_pk = kp.sign(*kp.public_key());
//    wa!(client.register_user(user_id, signed_pk));
//
//    let pending = wa!(client.get_pending(*kp.public_key(), 1));
//
//    assert_eq!(pending.len(), 0);
//
//    let push1 = Push {
//        tag: PushTag::User,
//        timestamp: Time::now(),
//        msg: bytes::Bytes::from_static(b"a"),
//    };
//
//    std::thread::sleep(std::time::Duration::from_secs(1));
//
//    let push2 = Push {
//        tag: PushTag::User,
//        timestamp: Time::now(),
//        msg: bytes::Bytes::from_static(b"b"),
//    };
//
//    std::thread::sleep(std::time::Duration::from_secs(1));
//
//    let push3 = Push {
//        tag: PushTag::User,
//        timestamp: Time::now(),
//        msg: bytes::Bytes::from_static(b"c"),
//    };
//
//    let addf = |p: Push| {
//        let pk = *kp.public_key();
//        std::thread::spawn(move || {
//            let mut rt = w!(Runtime::new());
//            rt.block_on(async {
//                let mut client = wa!(get_client());
//                assert!(client.add_pending(vec![pk], &[p]).await.is_ok());
//            });
//        })
//    };
//
//    let h1 = addf(push1.clone());
//    let h2 = addf(push2.clone());
//    let h3 = addf(push3.clone());
//
//    h1.join().expect(womp!("first insert failed"));
//    h2.join().expect(womp!("second insert failed"));
//    h3.join().expect(womp!("third insert failed"));
//
//    let mut pushes = vec![push1, push2, push3];
//    let pushes_unsorted = pushes.clone();
//    pushes.sort_unstable_by_key(|p| p.timestamp);
//    assert_eq!(pushes, pushes_unsorted);
//
//    let pending = wa!(client.get_pending(*kp.public_key(), 1));
//    assert_eq!(pending.as_slice(), &pushes[..1]);
//
//    let pending = wa!(client.get_pending(*kp.public_key(), 2));
//    assert_eq!(pending.as_slice(), &pushes[..2]);
//
//    let pending = wa!(client.get_pending(*kp.public_key(), 3));
//
//    assert_eq!(pending.as_slice(), &pushes[..3]);
//
//    let pending = wa!(client.get_pending(*kp.public_key(), 4));
//
//    assert_eq!(pending.as_slice(), &pushes[..3]);
//
//    wa!(client.expire_pending(*kp.public_key(), 1));
//
//    let pending = wa!(client.get_pending(*kp.public_key(), 1));
//
//    assert_eq!(pending.as_slice(), &pushes[1..2]);
//
//    let pending = wa!(client.get_pending(*kp.public_key(), 2));
//
//    assert_eq!(pending.as_slice(), &pushes[1..3]);
//
//    let pending = wa!(client.get_pending(*kp.public_key(), 3));
//
//    assert_eq!(pending.as_slice(), &pushes[1..3]);
//
//    assert!(client.expire_pending(*kp.public_key(), 2).await.is_ok());
//
//    let pending = wa!(client.get_pending(*kp.public_key(), 1));
//    assert!(pending.is_empty());
//}
//
//#[tokio::test]
//#[serial]
//async fn add_get_expire_pending_id() {
//    for _ in 0..10 {
//        let mut client = wa!(get_client());
//        wa!(client.reset_all());
//
//        let kp = sig::KeyPair::gen_new();
//        let user_id = "Hello".try_into().expect(womp!());
//
//        let signed_pk = kp.sign(*kp.public_key());
//        wa!(client.register_user(user_id, signed_pk));
//
//        let pending = wa!(client.get_pending(*kp.public_key(), 1));
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
//        wa!(client.add_pending(vec![*kp.public_key()], &pushes));
//
//        let pending = wa!(client.get_pending(*kp.public_key(), 1));
//        assert_eq!(pending.as_slice(), &pushes[..1]);
//
//        let pending = wa!(client.get_pending(*kp.public_key(), 2));
//        assert_eq!(pending.as_slice(), &pushes[..2]);
//
//        let pending = wa!(client.get_pending(*kp.public_key(), 3));
//        assert_eq!(pending.as_slice(), &pushes[..3]);
//
//        let pending = wa!(client.get_pending(*kp.public_key(), 4));
//        assert_eq!(pending.as_slice(), &pushes[..3]);
//
//        assert!(client.expire_pending(*kp.public_key(), 1).await.is_ok());
//
//        let pending = wa!(client.get_pending(*kp.public_key(), 1));
//        assert_eq!(pending.as_slice(), &pushes[1..2]);
//
//        let pending = wa!(client.get_pending(*kp.public_key(), 2));
//        assert_eq!(pending.as_slice(), &pushes[1..3]);
//
//        let pending = wa!(client.get_pending(*kp.public_key(), 3));
//        assert_eq!(pending.as_slice(), &pushes[1..3]);
//
//        assert!(client.expire_pending(*kp.public_key(), 2).await.is_ok());
//
//        let pending = wa!(client.get_pending(*kp.public_key(), 1));
//        assert!(pending.is_empty());
//    }
//}
//
//// #[tokio::test]
//// #[serial]
//// async fn add_and_get_prekey() {
////     let mut client = wa!(get_client());
////     wa!(client.reset_all());
//
////     let kp = sig::KeyPair::gen_new();
////     let signed_pk = kp.sign(*kp.public_key());
////     let user_id = "Hello".try_into().expect(womp!());
////     wa!(client.register_user(user_id, signed_pk));
//
////     let sealed_kp1 = sealed::KeyPair::gen_new();
////     let sealed_kp2 = sealed::KeyPair::gen_new();
//
////     let sealed_pk1 = sealed_kp1.sign_pub(&kp);
////     let sealed_pk2 = sealed_kp2.sign_pub(&kp);
//
////     wa!(client.add_prekeys(&[sealed_pk1, sealed_pk2]));
//
////     let retrieved = wa!(client.pop_prekeys(&[*kp.public_key()]))[0].expect(womp!());
////     assert!(retrieved == sealed_pk1 || retrieved == sealed_pk2);
//
////     let retrieved = wa!(client.pop_prekeys(&[*kp.public_key()]))[0].expect(womp!());
////     assert!(retrieved == sealed_pk1 || retrieved == sealed_pk2);
//
////     assert!(wa!(client.pop_prekeys(&[*kp.public_key()]))[0].is_none());
//// }
//
//#[tokio::test]
//#[serial]
//async fn key_is_valid() {
//    let mut client = wa!(get_client());
//    wa!(client.reset_all());
//
//    let kp = sig::KeyPair::gen_new();
//    let user_id = "Hello".try_into().expect(womp!());
//
//    let signed_pk = kp.sign(*kp.public_key());
//    assert!(!wa!(client.key_is_valid(*kp.public_key())));
//
//    wa!(client.register_user(user_id, signed_pk));
//
//    assert!(wa!(client.key_is_valid(*kp.public_key())));
//
//    let signed_pk = kp.sign(*kp.public_key());
//
//    wa!(client.deprecate_key(signed_pk));
//
//    assert!(!wa!(client.key_is_valid(*kp.public_key())));
//}
//
//#[tokio::test]
//#[serial]
//async fn double_deprecation() {
//    let mut client = wa!(get_client());
//    wa!(client.reset_all());
//
//    let kp1 = sig::KeyPair::gen_new();
//    let user_id = "hello".try_into().expect(womp!());
//
//    let signed_pk1 = kp1.sign(*kp1.public_key());
//
//    wa!(client.register_user(user_id, signed_pk1));
//
//    let kp2 = sig::KeyPair::gen_new();
//    let signed_pk2 = kp1.sign(*kp2.public_key());
//
//    wa!(client.add_key(signed_pk2));
//
//    wa!(client.deprecate_key(signed_pk2));
//
//    match client.deprecate_key(signed_pk2).await {
//        Ok(PKIResponse::Redundant) => {}
//        _ => panic!(),
//    }
//}
//
//#[tokio::test]
//#[serial]
//async fn invalid_deprecation() {
//    let mut client = wa!(get_client());
//    wa!(client.reset_all());
//
//    let kp1 = sig::KeyPair::gen_new();
//    let user_id = "hello".try_into().expect(womp!());
//
//    let signed_pk1 = kp1.sign(*kp1.public_key());
//
//    wa!(client.register_user(user_id, signed_pk1));
//
//    let kp2 = sig::KeyPair::gen_new();
//    let signed_pk2 = kp1.sign(*kp2.public_key());
//
//    wa!(client.add_key(signed_pk2));
//
//    wa!(client.deprecate_key(signed_pk1));
//
//    match client.deprecate_key(signed_pk2).await {
//        Ok(PKIResponse::DeadKey) => {}
//        _ => panic!(),
//    }
//}
