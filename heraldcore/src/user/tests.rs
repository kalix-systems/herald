use super::*;
use crate::db::Database;
use platform_dirs::pictures_dir;
use serial_test_derive::serial;
use std::convert::TryInto;

#[test]
fn add_user() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let id1 = "Hello".try_into().expect(womp!());
    let id2 = "World".try_into().expect(womp!());

    UserBuilder::new(id1)
        .name("name".into())
        .add_db(&mut conn)
        .expect("Failed to add user");

    UserBuilder::new(id2)
        .color(1)
        .add_db(&mut conn)
        .expect("Failed to add user");
}

#[test]
fn get_user_name() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());

    UserBuilder::new(id)
        .name("name".into())
        .add_db(&mut conn)
        .expect("Failed to add user");

    assert_eq!(
        db::name(&conn, id)
            .expect("Failed to get name")
            .expect(womp!()),
        "name"
    );
}

#[test]
#[serial(fs)]
fn fs_profile_picture() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());

    let test_picture = "test_resources/maryland.png";

    UserBuilder::new(id)
        .add_db(&mut conn)
        .expect(womp!("Failed to add user"));

    db::set_profile_picture(
        &conn,
        id,
        Some(image_utils::ProfilePicture::autocrop(test_picture.into())),
    )
    .expect(womp!("Failed to set profile picture"));

    std::fs::remove_dir_all(pictures_dir()).expect(womp!());
}

#[test]
fn get_set_color() {
    let mut conn = Database::in_memory_with_config().expect(womp!());
    let id = "userid".try_into().expect(womp!());

    UserBuilder::new(id)
        .name("Hello".into())
        .add_db(&mut conn)
        .expect(womp!());

    db::set_color(&conn, id, 1).expect("Failed to set color");

    let users = db::all(&conn).expect(womp!());

    assert_eq!(users[1].color, 1);
}

#[test]
fn check_user_exists() {
    let mut conn = Database::in_memory_with_config().expect(womp!());
    let id = "userid".try_into().expect(womp!());

    UserBuilder::new(id)
        .name("Hello".into())
        .add_db(&mut conn)
        .expect(womp!());

    assert_eq!(db::user_exists(&conn, id).unwrap(), true);
}

#[test]
fn update_name() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let id = "userid".try_into().expect(womp!());

    UserBuilder::new(id)
        .name("Hello".into())
        .add_db(&mut conn)
        .expect(womp!());

    db::set_name(&conn, id, "World".into()).expect("Failed to update name");

    assert_eq!(
        db::name(&conn, id)
            .expect("Failed to get user")
            .expect(womp!()),
        "World"
    );
}

#[test]
fn test_by_user_id() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let id = "id".try_into().expect(womp!());

    UserBuilder::new(id)
        .name("name".into())
        .add_db(&mut conn)
        .expect(womp!());

    let user = db::by_user_id(&conn, id).expect("Unable to get user from userid");

    assert_eq!(user.id, id);
    assert_eq!(user.name.as_str(), "name");
}

#[test]
fn all_users() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let id1 = "Hello".try_into().expect(womp!());
    let id2 = "World".try_into().expect(womp!());

    UserBuilder::new(id2)
        .add_db(&mut conn)
        .expect(womp!("Failed to add id1"));

    UserBuilder::new(id1)
        .add_db(&mut conn)
        .expect(womp!("Failed to add id2"));

    let users = db::all(&conn).expect(womp!());
    assert_eq!(users.len(), 3);
    assert_eq!(users[1].id, id1);
    assert_eq!(users[2].id, id2);
}

#[test]
fn add_remove_member() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let id1 = "id1".try_into().expect(womp!());
    let id2 = "id2".try_into().expect(womp!());

    let conv_id = ConversationId::from([0; 32]);

    UserBuilder::new(id1)
        .add_db(&mut conn)
        .expect(womp!("Failed to add id1"));

    UserBuilder::new(id2)
        .pairwise_conversation(conv_id)
        .add_db(&mut conn)
        .expect(womp!("Failed to add id2"));

    let users = db::all(&conn).expect(womp!());

    crate::members::db::add_member(&conn, &conv_id, &users[0].id)
        .expect(womp!("failed to add member"));

    let members = db::conversation_members(&conn, &conv_id).expect(womp!("failed to get members"));

    assert_eq!(members.len(), 2);

    assert!(members[0].id == id2 || members[1].id == id2);

    crate::members::db::remove_member(&conn, &conv_id, users[0].id)
        .expect(womp!("failed to remove member"));

    let members_new =
        db::conversation_members(&conn, &conv_id).expect(womp!("failed to get members"));

    assert_eq!(members_new.len(), 1);

    //is the correct member remaining?
    assert_eq!(members_new[0].id, id2);
}

#[test]
fn by_status_users() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let id1 = "Hello".try_into().expect(womp!());
    let id2 = "World".try_into().expect(womp!());

    UserBuilder::new(id1)
        .add_db(&mut conn)
        .expect("Failed to add id1");
    UserBuilder::new(id2)
        .status(UserStatus::Deleted)
        .add_db(&mut conn)
        .expect("Failed to add id2");

    let users = db::get_by_status(&conn, UserStatus::Active).expect(womp!());
    assert_eq!(users.len(), 2);
    assert!(users[0].id == id1 || users[1].id == id1 || users[2].id == id1);
}
