use super::*;
use crate::contact::ContactBuilder;
use crate::womp;
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
    deprecate_keys(uid, &[pk_signed]).expect(womp!());
}
