use super::*;
use crate::{contact::ContactBuilder, db::Database, womp};
use serial_test_derive::serial;
use std::convert::TryInto;

#[test]
#[serial]
fn conv_id_length() {
    Database::reset_all().expect(womp!());
    super::add_conversation(None, None).expect(womp!("failed to create conversation"));

    let all_meta = super::all_meta().expect(womp!("failed to get data"));

    assert_eq!(all_meta[0].conversation_id.into_array().len(), 32);
}

#[test]
#[serial]
fn add_conversation() {
    Database::reset_all().expect(womp!());

    // test without id
    super::add_conversation(None, None).expect(womp!("failed to create conversation"));

    let conversation_id = ConversationId::from([0; 32]);
    // test with id
    assert_eq!(
        conversation_id,
        super::add_conversation(Some(&conversation_id), None)
            .expect(womp!("failed to create conversation"))
    );

    super::add_conversation(Some(&[1; 32].into()), Some("el groupo"))
        .expect(womp!("failed to create conversation"));

    super::add_conversation(Some(&[2; 32].into()), Some("el groupo"))
        .expect(womp!("failed to create conversation"));
}

#[test]
#[serial]
fn add_and_get() {
    Database::reset_all().expect(womp!());

    let author = "Hello".try_into().unwrap();
    ContactBuilder::new(author).add().expect(womp!());

    let conversation = ConversationId::from([0; 32]);

    super::add_conversation(Some(&conversation), None)
        .expect(womp!("Failed to create conversation"));

    crate::message::add_message(None, author, &conversation, "1", None, None, &None)
        .expect(womp!("Failed to add first message"));

    crate::message::add_message(None, author, &conversation, "2", None, None, &None)
        .expect(womp!("Failed to add second message"));

    let msgs = super::conversation(&conversation).expect(womp!("Failed to get conversation"));

    assert_eq!(msgs.len(), 2);
}

#[test]
#[serial]
fn matches() {
    Database::reset_all().expect(womp!());

    // test without id
    let conv_id =
        super::add_conversation(None, Some("title")).expect(womp!("failed to create conversation"));

    let conv = meta(&conv_id).expect(womp!());

    let pattern = utils::SearchPattern::new_normal("titl".into()).expect(womp!());

    let bad_pattern = utils::SearchPattern::new_normal("tilt".into()).expect(womp!());

    assert_eq!(conv.matches(&pattern), true);

    assert_eq!(conv.matches(&bad_pattern), false);
}

#[test]
#[serial]
fn set_prof_pic() {
    Database::reset_all().expect(womp!());
    let conv_id = ConversationId::from([0; 32]);

    super::add_conversation(Some(&conv_id), None).expect(womp!("Failed to create conversation"));

    let test_picture = "test_resources/maryland.png";

    super::set_picture(&conv_id, Some(&test_picture), None).expect(womp!("failed to set picture"));

    std::fs::remove_dir_all("profile_pictures").expect(womp!());
}

#[test]
#[serial]
fn set_muted_test() {
    Database::reset_all().expect(womp!());
    let conv_id = ConversationId::from([0; 32]);

    super::add_conversation(Some(&conv_id), None).expect(womp!("Failed to create conversation"));

    super::set_muted(&conv_id, true).expect(womp!("Unable to set mute"));

    let meta = super::meta(&conv_id).expect(womp!("failed to get meta"));

    assert_eq!(meta.muted, true);

    super::set_muted(&conv_id, false).expect(womp!("Unable to set mute"));

    let meta = super::meta(&conv_id).expect(womp!("failed to get meta"));

    assert_eq!(meta.muted, false);
}

#[test]
#[serial]
fn set_get_meta() {
    Database::reset_all().expect(womp!());

    let conv_id = ConversationId::from([0; 32]);

    super::add_conversation(Some(&conv_id), None).expect(womp!("Failed to create conversation"));

    super::set_color(&conv_id, 1).expect(womp!("Failed to set color"));

    super::set_title(&conv_id, Some("title")).expect(womp!("Failed to set title"));

    let conv_meta = super::meta(&conv_id).expect(womp!("Failed to get metadata"));

    assert_eq!(conv_meta.conversation_id, conv_id);
    assert_eq!(conv_meta.title.expect("failed to get title"), "title");
    assert_eq!(conv_meta.color, 1);

    let conv_id2 = ConversationId::from([1; 32]);

    super::add_conversation(Some(&conv_id2), Some("hello"))
        .expect(womp!("Failed to create conversation"));

    let all_meta = super::all_meta().expect(womp!("Failed to get all metadata"));

    assert_eq!(all_meta.len(), 2);

    assert_eq!(all_meta[1].conversation_id, conv_id2);
}

#[test]
#[serial]
fn conv_messages_since() {
    Database::reset_all().expect(womp!());

    let contact = "contact".try_into().unwrap();
    ContactBuilder::new(contact).add().expect(womp!());

    let conv_id = ConversationId::from([0; 32]);

    super::add_conversation(Some(&conv_id), None).expect(womp!("Failed to make conversation"));

    crate::message::add_message(None, contact, &conv_id, "1", None, None, &None)
        .expect(womp!("Failed to make message"));
    let timestamp = chrono::Utc::now();

    assert!(conversation_messages_since(&conv_id, timestamp)
        .expect(womp!())
        .is_empty());
}

#[test]
#[serial]
fn add_remove_member() {
    Database::reset_all().expect(womp!());

    let id1 = "id1".try_into().unwrap();
    let id2 = "id2".try_into().unwrap();

    let conv_id = ConversationId::from([0; 32]);

    ContactBuilder::new(id1)
        .add()
        .expect(womp!("Failed to add id1"));

    ContactBuilder::new(id2)
        .add()
        .expect(womp!("Failed to add id2"));

    super::add_conversation(Some(&conv_id), None).expect(womp!("Failed to create conversation"));

    crate::members::add_member(&conv_id, id1).expect(womp!("failed to add member"));

    crate::members::add_member(&conv_id, id2).expect(womp!("failed to add member"));

    let members = crate::members::members(&conv_id).expect(womp!("failed to get members"));

    assert_eq!(members.len(), 2);

    crate::members::remove_member(&conv_id, id2).expect(womp!("failed to remove member"));

    let members = crate::members::members(&conv_id).expect(womp!("failed to get members"));

    assert_eq!(members.len(), 1);
}

#[test]
#[serial]
fn delete_message() {
    Database::reset_all().expect(womp!());

    let author = "Hello".try_into().unwrap();
    ContactBuilder::new(author).add().expect(womp!());

    let conversation = ConversationId::from([0; 32]);

    super::add_conversation(Some(&conversation), None)
        .expect(womp!("Failed to create conversation"));

    let (msg_id, _) =
        crate::message::add_message(None, author, &conversation, "1", None, None, &None)
            .expect(womp!("Failed to add first message"));

    crate::message::delete_message(&msg_id).expect(womp!());

    assert!(super::conversation(&conversation)
        .expect(womp!())
        .is_empty());
}

#[test]
#[serial]
fn delete_conversation() {
    Database::reset_all().expect(womp!());

    let author = "Hello".try_into().unwrap();
    ContactBuilder::new(author).add().expect(womp!());

    let conversation = [0; 32].into();

    super::add_conversation(Some(&conversation), None)
        .expect(womp!("Failed to create conversation"));

    crate::message::add_message(None, author, &conversation, "1", None, None, &None)
        .expect(womp!("Failed to add first message"));

    crate::message::add_message(None, author, &conversation, "1", None, None, &None)
        .expect(womp!("Failed to add second message"));

    super::delete_conversation(&conversation).expect(womp!());

    assert!(super::conversation(&conversation)
        .expect(womp!())
        .is_empty());
}
