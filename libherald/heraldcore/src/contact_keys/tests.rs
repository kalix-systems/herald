use super::*;
use crate::womp;
use herald_common::sig;
use serial_test_derive::serial;

#[test]
#[serial]
fn add_and_deprecate() {
    Database::reset_all().expect(womp!());
    let kp = sig::KeyPair::gen_new();
    let pk_signed = kp.sign(*kp.public_key());

    add_key(pk_signed).expect(womp!());
    key_deprecated(pk_signed).expect(womp!());
}
