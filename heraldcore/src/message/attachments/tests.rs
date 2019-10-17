use super::*;
use crate::{config::test_config, womp};
use serial_test_derive::serial;
use std::str::FromStr;

#[test]
#[serial]
fn make_attachment() {
    let path = PathBuf::from_str("test_resources/maryland.png").expect(womp!());
    let attach = Attachment::new(&path).expect(womp!());

    let out_path = attach.save().expect(womp!());
    std::fs::remove_dir_all(out_path).expect(womp!());
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
        .into_vector_of_strings()
        .expect(womp!());

    assert_eq!(meta.len(), 1);
    std::fs::remove_dir_all("attachments").expect(womp!());
}

#[test]
#[serial]
fn inbound_message_attachment() {
    use crate::contact::ContactBuilder;
    use std::convert::TryInto;

    Database::reset_all().expect(womp!());
    test_config();

    let other = ContactBuilder::new("hi".try_into().expect(womp!()))
        .add()
        .expect(womp!());
    let path = PathBuf::from_str("test_resources/maryland.png").expect(womp!());
    let attach = Attachment::new(&path).expect(womp!());

    let mid = [0; 32].into();
    let mut builder = InboundMessageBuilder::default();
    builder
        .id(mid)
        .author(other.id)
        .timestamp(Utc::now())
        .attachments(vec![attach])
        .conversation_id(other.pairwise_conversation);
    builder.store().expect(womp!());

    let meta = super::get(&mid)
        .expect(womp!())
        .into_vector_of_strings()
        .expect(womp!());
    assert_eq!(meta.len(), 1);

    std::fs::remove_dir_all("attachments").expect(womp!());
}
