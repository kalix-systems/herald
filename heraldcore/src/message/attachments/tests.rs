use super::*;
use crate::{
    config::{self, test_config},
    womp,
};
use serial_test_derive::serial;
use std::convert::TryInto;
use std::str::FromStr;

#[test]
#[serial(attach)]
fn make_attachment() {
    let path = PathBuf::from_str("test_resources/maryland.png").expect(womp!());
    let attach = Attachment::new(&path).expect(womp!());

    let path = ATTACHMENTS_DIR.join(attach.save().expect(womp!()));
    std::fs::remove_dir_all(path).expect(womp!());
    std::fs::remove_dir_all(ATTACHMENTS_DIR.as_path()).expect(womp!());
}

#[test]
#[serial(attach)]
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
        .into_flat_strings()
        .expect(womp!());

    assert_eq!(meta.len(), 1);
    std::fs::remove_dir_all(ATTACHMENTS_DIR.as_path()).expect(womp!());
}

#[test]
#[serial(attach)]
fn inbound_message_attachment() {
    use crate::contact::ContactBuilder;
    use std::convert::TryInto;

    let mut conn = Database::in_memory().expect(womp!());
    config::db::test_config(&mut conn);

    let other = ContactBuilder::new("hi".try_into().expect(womp!()))
        .add_db(&mut conn)
        .expect(womp!());

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
    let meta = meta.into_flat_strings().expect(womp!());

    assert_eq!(meta.len(), 1);

    std::fs::remove_dir_all(ATTACHMENTS_DIR.as_path()).expect(womp!());
}

#[test]
#[serial(attach)]
fn delete_message_with_attachment() {
    let mut conn = Database::in_memory().expect(womp!());

    let receiver = crate::contact::db::test_contact(&mut conn, "receiver");

    let path = PathBuf::from_str("test_resources/maryland.png").expect(womp!());

    let attach = Attachment::new(&path).expect(womp!());
    let attach_path = ATTACHMENTS_DIR.join(attach.hash_dir());

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

    std::fs::remove_dir_all(ATTACHMENTS_DIR.as_path()).expect(womp!());
}
