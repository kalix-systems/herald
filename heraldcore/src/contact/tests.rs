use super::*;
use crate::{db::Database, womp};
use serial_test_derive::serial;

#[test]
#[serial]
fn add_contact() {
    Database::reset_all().expect(womp!());

    let id1 = "Hello".try_into().expect(womp!());
    let id2 = "World".try_into().expect(womp!());

    ContactBuilder::new(id1)
        .name("name".into())
        .add()
        .expect("Failed to add contact");
    ContactBuilder::new(id2)
        .color(1)
        .add()
        .expect("Failed to add contact");
}

#[test]
#[serial]
fn get_contact_name() {
    Database::reset_all().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());

    ContactBuilder::new(id)
        .name("name".into())
        .add()
        .expect("Failed to add contact");

    assert_eq!(
        name(id).expect("Failed to get name").expect(womp!()),
        "name"
    );
}

#[test]
#[serial]
fn get_set_contact_profile_picture() {
    Database::reset_all().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());
    let profile_picture = "picture";

    ContactBuilder::new(id)
        .profile_picture(profile_picture.into())
        .add()
        .expect("Failed to add contact");

    assert_eq!(
        super::profile_picture(id)
            .expect("Failed to get profile picture")
            .expect(womp!())
            .as_str(),
        profile_picture
    );

    Database::reset_all().expect(womp!());

    let test_picture = "test_resources/maryland.png";

    ContactBuilder::new(id)
        .add()
        .expect("Failed to add contact");

    set_profile_picture(id, Some(test_picture.into()), None)
        .expect("Failed to set profile picture");

    std::fs::remove_dir_all("profile_pictures").expect(womp!());
}

#[test]
#[serial]
fn get_set_color() {
    Database::reset_all().expect(womp!());
    let id = "userid".try_into().expect(womp!());

    ContactBuilder::new(id)
        .name("Hello".into())
        .add()
        .expect(womp!());

    set_color(id, 1).expect("Failed to set color");

    let contacts = all().expect(womp!());

    assert_eq!(contacts[0].color, 1);
}

#[test]
#[serial]
fn check_contact_exists() {
    Database::reset_all().expect(womp!());
    let id = "userid".try_into().expect(womp!());

    ContactBuilder::new(id)
        .name("Hello".into())
        .add()
        .expect(womp!());

    assert_eq!(contact_exists(id).unwrap(), true);

    Database::reset_all().expect(womp!());

    assert_eq!(contact_exists(id).unwrap(), false)
}

#[test]
#[serial]
fn update_name() {
    Database::reset_all().expect(womp!());

    let id = "userid".try_into().expect(womp!());

    ContactBuilder::new(id)
        .name("Hello".into())
        .add()
        .expect(womp!());

    set_name(id, Some("World")).expect("Failed to update name");

    assert_eq!(
        name(id).expect("Failed to get contact").expect(womp!()),
        "World"
    );
}

#[test]
#[serial]
fn test_by_user_id() {
    Database::reset_all().expect(womp!());

    let id = "id".try_into().expect(womp!());

    ContactBuilder::new(id)
        .name("name".into())
        .add()
        .expect(womp!());

    let contact = by_user_id(id).expect("Unable to get contact from userid");

    assert_eq!(contact.id, id);
    assert_eq!(contact.name.expect(womp!()), "name");
}

#[test]
#[serial]
fn all_contacts() {
    Database::reset_all().expect(womp!());

    let id1 = "Hello".try_into().expect(womp!());
    let id2 = "World".try_into().expect(womp!());

    ContactBuilder::new(id1)
        .add()
        .expect(womp!("Failed to add id1"));

    ContactBuilder::new(id2)
        .add()
        .expect(womp!("Failed to add id2"));

    let contacts = all().expect(womp!());
    assert_eq!(contacts.len(), 2);
    assert_eq!(contacts[0].id, id1);
    assert_eq!(contacts[1].id, id2);
}

#[test]
#[serial]
fn set_status() {
    Database::reset_all().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());
    let contact = ContactBuilder::new(id).add().expect(womp!());

    super::set_status(id, ContactStatus::Archived).expect(womp!());

    assert_eq!(
        status(id).expect("Failed to determine contact status"),
        ContactStatus::Archived
    );

    super::set_status(id, ContactStatus::Deleted).expect(womp!());

    assert_eq!(
        super::status(id).expect("Failed to determine contact status"),
        ContactStatus::Deleted
    );

    assert!(
        crate::conversation::conversation_messages(&contact.pairwise_conversation)
            .expect(womp!())
            .is_empty()
    );
}

#[test]
#[serial]
fn add_remove_member() {
    Database::reset_all().expect(womp!());

    let id1 = "id1".try_into().expect(womp!());
    let id2 = "id2".try_into().expect(womp!());

    let conv_id = ConversationId::from([0; 32]);

    ContactBuilder::new(id1)
        .add()
        .expect(womp!("Failed to add id1"));

    ContactBuilder::new(id2)
        .pairwise_conversation(conv_id)
        .add()
        .expect(womp!("Failed to add id2"));

    let contacts = all().expect(womp!());

    crate::members::add_member(&conv_id, contacts[0].id).expect(womp!("failed to add member"));

    let members = conversation_members(&conv_id).expect(womp!("failed to get members"));

    assert_eq!(members.len(), 2);

    assert_eq!(members[0].id, id1);

    crate::members::remove_member(&conv_id, contacts[0].id)
        .expect(womp!("failed to remove member"));

    let members_new = conversation_members(&conv_id).expect(womp!("failed to get members"));

    assert_eq!(members_new.len(), 1);
    //is the correct member remaining?
    assert_eq!(members_new[0].id, id2);
}

#[test]
#[serial]
fn by_status_contacts() {
    Database::reset_all().expect(womp!());

    let id1 = "Hello".try_into().expect(womp!());
    let id2 = "World".try_into().expect(womp!());

    ContactBuilder::new(id1).add().expect("Failed to add id1");
    ContactBuilder::new(id2)
        .status(ContactStatus::Archived)
        .add()
        .expect("Failed to add id2");

    let contacts = get_by_status(ContactStatus::Active).expect(womp!());
    assert_eq!(contacts.len(), 1);
    assert_eq!(contacts[0].id, id1);
}
