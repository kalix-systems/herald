use super::*;
use crate::config::test_config;
use herald_attachments::Attachment;
use platform_dirs::attachments_dir;
use serial_test_derive::serial;
use std::{convert::TryInto, str::FromStr};

#[test]
#[serial]
fn make_attachment() {
    let path = PathBuf::from_str("test_resources/maryland.png").expect(womp!());
    let attach = Attachment::new(&path).expect(womp!());

    let path = attachments_dir().join(attach.save().expect(womp!()));
    std::fs::remove_dir_all(path).expect(womp!());
    std::fs::remove_dir_all(attachments_dir()).expect(womp!());
}

#[test]
#[serial]
fn outbound_message_attachment() {
    Database::reset_all().expect(womp!());
    let path = PathBuf::from_str("test_resources/maryland.png").expect(womp!());
    let config = test_config();

    let mut builder = OutboundMessageBuilder::default();
    builder
        .add_attachment((&path).clone())
        .conversation_id(config.nts_conversation);
    let msg = builder.store_and_send_blocking().expect(womp!());

    let meta = super::get(&msg.message_id)
        .expect(womp!())
        .flat()
        .expect(womp!());

    assert_eq!(meta.len(), 1);
    std::fs::remove_dir_all(attachments_dir()).expect(womp!());
}

#[test]
#[serial]
fn inbound_message_attachment() {
    use crate::user::UserBuilder;
    use std::convert::TryInto;

    let mut conn = Database::in_memory_with_config().expect(womp!());

    let other = UserBuilder::new("hi".try_into().expect(womp!()))
        .add_db(&mut conn)
        .expect(womp!())
        .0;

    let path = PathBuf::from_str("test_resources/maryland.png").expect(womp!());
    let attach = Attachment::new(&path).expect(womp!());

    let mid = [0; 32].into();
    let mut builder = InboundMessageBuilder::default();
    builder
        .id(mid)
        .author(other.id)
        .timestamp(Time::now())
        .attachments(vec![attach])
        .conversation_id(other.pairwise_conversation);
    builder.store_db(&mut conn).expect(womp!());

    let meta = db::get(&conn, &mid).expect(womp!());
    let meta = meta.flat().expect(womp!());

    assert_eq!(meta.len(), 1);

    std::fs::remove_dir_all(attachments_dir()).expect(womp!());
}

#[test]
#[serial]
fn delete_message_with_attachment() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let receiver = crate::user::db::test_user(&mut conn, "receiver");

    let path = PathBuf::from_str("test_resources/maryland.png").expect(womp!());

    let attach = Attachment::new(&path).expect(womp!());
    let attach_path = attachments_dir().join(attach.hash_dir());

    let conv = receiver.pairwise_conversation;

    let mid0 = [0; 32].into();

    let mut builder = InboundMessageBuilder::default();
    builder
        .id(mid0)
        .author(receiver.id)
        .conversation_id(conv)
        .attachments(vec![(&attach).clone()])
        .timestamp(Time::now())
        .body("hi".try_into().expect(womp!()));

    builder.store_db(&mut conn).expect(womp!());

    let mid1 = [1; 32].into();

    let mut builder = InboundMessageBuilder::default();
    builder
        .id(mid1)
        .author(receiver.id)
        .conversation_id(conv)
        .attachments(vec![(&attach).clone()])
        .timestamp(Time::now())
        .body("hi".try_into().expect(womp!()));

    builder.store_db(&mut conn).expect(womp!());

    assert!(attach_path.exists());

    crate::message::db::delete_message(&conn, &mid0).expect(womp!());

    assert!(attach_path.exists());

    crate::message::db::delete_message(&conn, &mid1).expect(womp!());

    assert!(!attach_path.exists());

    std::fs::remove_dir_all(attachments_dir()).expect(womp!());
}
