use super::*;
use crate::connection::in_memory;
use coremacros::womp;
use std::convert::TryInto;

#[test]
fn chain_ops() {
    let mut conn = in_memory();
    let mut conn = Conn::from(conn.transaction().expect(womp!()));

    let user_id: UserId = "a".try_into().expect(womp!());

    // should be None
    assert!(conn.get_sigchain(user_id).expect(womp!()).is_none());

    let kp1 = sig::KeyPair::gen_new();
    let init = sig::sign_ser(&kp1, user_id);

    conn.start_sigchain(init).expect(womp!());

    // should be None
    assert_eq!(
        conn.get_sigchain(user_id).expect(womp!()),
        Some(sig::SigChain {
            initial: init,
            sig_chain: vec![]
        })
    );

    assert!(conn.key_is_valid(*kp1.public(), user_id).expect(womp!()));
}
