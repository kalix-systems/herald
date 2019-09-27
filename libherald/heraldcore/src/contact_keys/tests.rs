use super::*;
use crate::womp;
use serial_test_derive::serial;

#[test]
#[serial]
fn create_drop_exists() {
    Database::reset_all().expect(womp!());
    // drop twice, it shouldn't panic on multiple drops
    ContactKeys::drop_table().expect(womp!());
    ContactKeys::drop_table().expect(womp!());

    ContactKeys::create_table().expect(womp!());
    assert!(ContactKeys::exists().expect(womp!()));
    ContactKeys::create_table().expect(womp!());
    assert!(ContactKeys::exists().expect(womp!()));
    ContactKeys::drop_table().expect(womp!());
    assert!(!ContactKeys::exists().expect(womp!()));
    ContactKeys::reset().expect(womp!());
}
