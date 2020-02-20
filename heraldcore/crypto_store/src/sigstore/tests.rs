use super::*;
use crate::connection::in_memory;
use coremacros::womp;
use std::convert::TryInto;

#[test]
fn chain_ops() {
    let set = |v: Vec<_>| v.into_iter().collect::<std::collections::BTreeSet<_>>();

    let mut conn = in_memory();
    let mut conn = Conn::from(conn.transaction().expect(womp!()));

    let user_id: UserId = "a".try_into().expect(womp!());
    let kp1 = sig::KeyPair::gen_new();

    // should be empty
    assert!(conn.all_active_keys().expect(womp!()).is_empty());

    // should be None
    assert!(conn.get_sigchain(user_id).expect(womp!()).is_none());
    // should be invalid
    assert!(!conn.key_is_valid(*kp1.public(), user_id).expect(womp!()));

    let init = sig::sign_ser(&kp1, user_id);
    conn.start_sigchain(init).expect(womp!());

    assert_eq!(
        conn.get_sigchain(user_id).expect(womp!()),
        Some(sig::SigChain {
            initial: init,
            sig_chain: vec![]
        })
    );
    assert!(conn.key_is_valid(*kp1.public(), user_id).expect(womp!()));
    assert_eq!(conn.all_active_keys().expect(womp!()), vec![*kp1.public()]);

    let kp2 = sig::KeyPair::gen_new();
    let endorse = sig::SigUpdate::Endorse(sig::sign_ser(&kp2, user_id));
    let signed_endorse = sig::sign_ser(&kp1, endorse);

    conn.extend_sigchain(user_id, signed_endorse.clone())
        .expect(womp!());

    assert!(conn.key_is_valid(*kp2.public(), user_id).expect(womp!()));
    assert_eq!(
        conn.get_sigchain(user_id).expect(womp!()),
        Some(sig::SigChain {
            initial: init,
            sig_chain: vec![signed_endorse]
        })
    );
    assert_eq!(
        set(conn.all_active_keys().expect(womp!())),
        set(vec![*kp1.public(), *kp2.public()])
    );

    // time resolution
    std::thread::sleep(std::time::Duration::from_millis(5));

    let deprecate = sig::SigUpdate::Deprecate(*kp1.public());
    let signed_deprecate = sig::sign_ser(&kp1, deprecate);

    conn.extend_sigchain(user_id, signed_deprecate.clone())
        .expect(womp!());

    assert_eq!(
        conn.get_sigchain(user_id).expect(womp!()),
        Some(sig::SigChain {
            initial: init,
            sig_chain: vec![signed_endorse, signed_deprecate]
        })
    );

    assert!(!conn.key_is_valid(*kp1.public(), user_id).expect(womp!()));
    assert_eq!(conn.all_active_keys().expect(womp!()), vec![*kp2.public()]);
    assert_eq!(conn.get_all_users().expect(womp!()), vec![user_id]);
}
