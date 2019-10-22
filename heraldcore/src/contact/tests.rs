use super::*;
use crate::{db::Database, womp};
use serial_test_derive::serial;

#[test]
fn add_contact() {
    let mut conn = Database::in_memory().expect(womp!());

    let id1 = "Hello".try_into().expect(womp!());
    let id2 = "World".try_into().expect(womp!());

    ContactBuilder::new(id1)
        .name("name".into())
        .add_db(&mut conn)
        .expect("Failed to add contact");
    ContactBuilder::new(id2)
        .color(1)
        .add_db(&mut conn)
        .expect("Failed to add contact");
}

#[test]
fn get_contact_name() {
    let mut conn = Database::in_memory().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());

    ContactBuilder::new(id)
        .name("name".into())
        .add_db(&mut conn)
        .expect("Failed to add contact");

    assert_eq!(
        db::name(&conn, id)
            .expect("Failed to get name")
            .expect(womp!()),
        "name"
    );
}

#[test]
fn contact_profile_picture() {
    let mut conn = Database::in_memory().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());
    let profile_picture = "picture";

    ContactBuilder::new(id)
        .profile_picture(profile_picture.into())
        .add_db(&mut conn)
        .expect("Failed to add contact");

    assert_eq!(
        db::profile_picture(&conn, id)
            .expect("Failed to get profile picture")
            .expect(womp!())
            .as_str(),
        profile_picture
    );
}

#[test]
#[serial(fs)]
fn fs_profile_picture() {
    let mut conn = Database::in_memory().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());

    let test_picture = "test_resources/maryland.png";

    ContactBuilder::new(id)
        .add_db(&mut conn)
        .expect("Failed to add contact");

    db::set_profile_picture(&conn, id, Some(test_picture.into()), None)
        .expect("Failed to set profile picture");

    std::fs::remove_dir_all("profile_pictures").expect(womp!());
}

#[test]
fn get_set_color() {
    let mut conn = Database::in_memory().expect(womp!());
    let id = "userid".try_into().expect(womp!());

    ContactBuilder::new(id)
        .name("Hello".into())
        .add_db(&mut conn)
        .expect(womp!());

    db::set_color(&conn, id, 1).expect("Failed to set color");

    let contacts = db::all(&conn).expect(womp!());

    assert_eq!(contacts[0].color, 1);
}

#[test]
fn check_contact_exists() {
    let mut conn = Database::in_memory().expect(womp!());
    let id = "userid".try_into().expect(womp!());

    ContactBuilder::new(id)
        .name("Hello".into())
        .add_db(&mut conn)
        .expect(womp!());

    assert_eq!(db::contact_exists(&conn, id).unwrap(), true);
}

#[test]
fn update_name() {
    let mut conn = Database::in_memory().expect(womp!());

    let id = "userid".try_into().expect(womp!());

    ContactBuilder::new(id)
        .name("Hello".into())
        .add_db(&mut conn)
        .expect(womp!());

    db::set_name(&conn, id, "World").expect("Failed to update name");

    assert_eq!(
        db::name(&conn, id)
            .expect("Failed to get contact")
            .expect(womp!()),
        "World"
    );
}

#[test]
fn test_by_user_id() {
    let mut conn = Database::in_memory().expect(womp!());

    let id = "id".try_into().expect(womp!());

    ContactBuilder::new(id)
        .name("name".into())
        .add_db(&mut conn)
        .expect(womp!());

    let contact = db::by_user_id(&conn, id).expect("Unable to get contact from userid");

    assert_eq!(contact.id, id);
    assert_eq!(contact.name.as_str(), "name");
}

#[test]
fn all_contacts() {
    let mut conn = Database::in_memory().expect(womp!());

    let id1 = "Hello".try_into().expect(womp!());
    let id2 = "World".try_into().expect(womp!());

    ContactBuilder::new(id2)
        .add_db(&mut conn)
        .expect(womp!("Failed to add id1"));

    ContactBuilder::new(id1)
        .add_db(&mut conn)
        .expect(womp!("Failed to add id2"));

    let contacts = db::all(&conn).expect(womp!());
    assert_eq!(contacts.len(), 2);
    assert_eq!(contacts[0].id, id1);
    assert_eq!(contacts[1].id, id2);
}

#[test]
fn set_status() {
    let mut conn = Database::in_memory().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());
    let contact = ContactBuilder::new(id).add_db(&mut conn).expect(womp!());

    db::set_status(&mut conn, id, ContactStatus::Archived).expect(womp!());

    assert_eq!(
        db::status(&conn, id).expect("Failed to determine contact status"),
        ContactStatus::Archived
    );

    db::set_status(&mut conn, id, ContactStatus::Deleted).expect(womp!());

    assert_eq!(
        db::status(&conn, id).expect("Failed to determine contact status"),
        ContactStatus::Deleted
    );

    assert!(
        crate::conversation::db::conversation_messages(&conn, &contact.pairwise_conversation)
            .expect(womp!())
            .is_empty()
    );
}

#[test]
fn add_remove_member() {
    let mut conn = Database::in_memory().expect(womp!());

    let id1 = "id1".try_into().expect(womp!());
    let id2 = "id2".try_into().expect(womp!());

    let conv_id = ConversationId::from([0; 32]);

    ContactBuilder::new(id1)
        .add_db(&mut conn)
        .expect(womp!("Failed to add id1"));

    ContactBuilder::new(id2)
        .pairwise_conversation(conv_id)
        .add_db(&mut conn)
        .expect(womp!("Failed to add id2"));

    let contacts = db::all(&conn).expect(womp!());

    crate::members::db::add_member(&conn, &conv_id, contacts[0].id)
        .expect(womp!("failed to add member"));

    let members = db::conversation_members(&conn, &conv_id).expect(womp!("failed to get members"));

    assert_eq!(members.len(), 2);

    assert_eq!(members[0].id, id1);

    crate::members::db::remove_member(&conn, &conv_id, contacts[0].id)
        .expect(womp!("failed to remove member"));

    let members_new =
        db::conversation_members(&conn, &conv_id).expect(womp!("failed to get members"));

    assert_eq!(members_new.len(), 1);
    //is the correct member remaining?
    assert_eq!(members_new[0].id, id2);
}

#[test]
fn by_status_contacts() {
    let mut conn = Database::in_memory().expect(womp!());

    let id1 = "Hello".try_into().expect(womp!());
    let id2 = "World".try_into().expect(womp!());

    ContactBuilder::new(id1)
        .add_db(&mut conn)
        .expect("Failed to add id1");
    ContactBuilder::new(id2)
        .status(ContactStatus::Archived)
        .add_db(&mut conn)
        .expect("Failed to add id2");

    let contacts = db::get_by_status(&conn, ContactStatus::Active).expect(womp!());
    assert_eq!(contacts.len(), 1);
    assert_eq!(contacts[0].id, id1);
}
