use super::*;
use crate::{db::Database, womp};
use serial_test_derive::serial;

#[test]
#[serial]
fn create_drop_exists_reset() {
    Database::reset_all().expect(womp!());
    // drop twice, it shouldn't panic on multiple drops
    Members::drop_table().expect(womp!());
    Members::drop_table().expect(womp!());

    Members::create_table().expect(womp!());
    assert!(Members::exists().expect(womp!()));
    Members::create_table().expect(womp!());
    assert!(Members::exists().expect(womp!()));
    Members::drop_table().expect(womp!());
    assert!(!Members::exists().expect(womp!()));

    Database::reset_all().expect(womp!());

    Members::create_table().expect(womp!());
    Members::reset().expect(womp!());
}
