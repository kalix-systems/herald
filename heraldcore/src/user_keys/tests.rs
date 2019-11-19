use super::*;
use crate::user::UserBuilder;
use herald_common::sig;
use std::convert::TryFrom;

#[test]
fn add_and_deprecate() {
    let mut conn = Database::in_memory_with_config().expect(womp!());
    let uid = UserId::try_from("Hello").expect(womp!());
    UserBuilder::new(uid).add_db(&mut conn).expect(womp!());

    let kp = sig::KeyPair::gen_new();
    let pk_signed = kp.sign(*kp.public_key());

    db::add_keys(&mut conn, uid, &[pk_signed]).expect(womp!());
    db::deprecate_keys(&mut conn, &[pk_signed]).expect(womp!());
}

#[test]
fn get_valid_deprecated() {
    let mut conn = Database::in_memory_with_config().expect(womp!());
    let uid = UserId::try_from("Hello").expect(womp!());
    UserBuilder::new(uid).add_db(&mut conn).expect(womp!());

    let kp1 = sig::KeyPair::gen_new();
    let kp2 = sig::KeyPair::gen_new();
    let pk1_signed = kp1.sign(*kp1.public_key());
    let pk2_signed = kp1.sign(*kp2.public_key());

    db::add_keys(&mut conn, uid, &[pk1_signed, pk2_signed]).expect(womp!());
    let valid = db::get_valid_keys(&conn, uid).expect(womp!());
    let dep_keys = db::get_deprecated_keys(&conn, uid).expect(womp!());
    assert_eq!(valid.len(), 2);
    assert!(dep_keys.is_empty());
    db::deprecate_keys(&mut conn, &[pk1_signed]).expect(womp!());

    let valid = db::get_valid_keys(&conn, uid).expect(womp!());
    let dep_keys = db::get_deprecated_keys(&conn, uid).expect(womp!());
    assert_eq!(valid.len(), 1);
    assert_eq!(dep_keys.len(), 1);
}
