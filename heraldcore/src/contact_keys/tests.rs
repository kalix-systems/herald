use super::*;
use crate::{contact::ContactBuilder, womp};
use herald_common::sig;
use serial_test_derive::serial;
use std::convert::TryFrom;

#[test]
#[serial]
fn add_and_deprecate() {
    Database::reset_all().expect(womp!());
    let uid = UserId::try_from("Hello").expect(womp!());
    ContactBuilder::new(uid).add().expect(womp!());

    let kp = sig::KeyPair::gen_new();
    let pk_signed = kp.sign(*kp.public_key());

    add_keys(uid, &[pk_signed]).expect(womp!());
    deprecate_keys(&[pk_signed]).expect(womp!());
}

#[test]
#[serial]
fn get_valid_deprecated() {
    Database::reset_all().expect(womp!());
    let uid = UserId::try_from("Hello").expect(womp!());
    ContactBuilder::new(uid).add().expect(womp!());

    let kp1 = sig::KeyPair::gen_new();
    let kp2 = sig::KeyPair::gen_new();
    let pk1_signed = kp1.sign(*kp1.public_key());
    let pk2_signed = kp1.sign(*kp2.public_key());

    add_keys(uid, &[pk1_signed, pk2_signed]).expect(womp!());
    let valid = get_valid_keys(uid).expect(womp!());
    let dep_keys = get_deprecated_keys(uid).expect(womp!());
    assert_eq!(valid.len(), 2);
    assert!(dep_keys.is_empty());
    deprecate_keys(&[pk1_signed]).expect(womp!());

    let valid = get_valid_keys(uid).expect(womp!());
    let dep_keys = get_deprecated_keys(uid).expect(womp!());
    assert_eq!(valid.len(), 1);
    assert_eq!(dep_keys.len(), 1);
}
