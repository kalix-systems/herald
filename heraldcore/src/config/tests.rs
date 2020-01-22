use super::*;
use herald_common::sig::KeyPair;
use platform_dirs::pictures_dir;
use serial_test_derive::serial;
use std::convert::TryInto;

#[test]
fn simple_add_get_set_config() {
    let mut conn = Database::in_memory().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());

    let kp = KeyPair::gen_new();
    ConfigBuilder::new(id, kp).add_db(&mut conn).expect(womp!());

    let config = db::get(&conn).expect(womp!());
    assert_eq!(config.id, id);
    assert_eq!(config.colorscheme, 0);
    assert_eq!(config.color, crate::utils::id_to_color(id));
    assert_eq!(config.color, crate::utils::id_to_color(id));
    assert_eq!(config.name.as_str(), id.as_str());
    assert!(config.profile_picture.is_none());
}

#[test]
#[serial(fs)]
fn complicated_add_get_set_config() {
    let mut conn = Database::in_memory().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());

    let name = "stuff";
    let nts_id = [0u8; 32].into();
    let kp = KeyPair::gen_new();
    let config = ConfigBuilder::new(id, kp)
        .name(name.into())
        .colorscheme(1)
        .color(2)
        .nts_conversation(nts_id)
        .add_db(&mut conn)
        .expect(womp!());

    let meta = crate::conversation::db::meta(&conn, &config.nts_conversation).expect(womp!());

    assert_eq!(meta.title.expect(womp!()), "stuff");

    let db_config = db::get(&conn).expect(womp!());

    assert_eq!(config.nts_conversation, db_config.nts_conversation);
    assert_eq!(db_config.id, id);
    assert_eq!(db_config.name.as_str(), name);
    assert_eq!(db_config.nts_conversation, nts_id);
    assert_eq!(db_config.colorscheme, 1);
    assert_eq!(db_config.color, 2);

    db::set_name(&conn, "test").expect(womp!());
    let db_config = db::get(&conn).expect(womp!());
    assert_eq!(db_config.name, "test");

    db::set_name(&conn, "hello").expect(womp!());

    let db_config = db::get(&conn).expect(womp!());
    assert_eq!(db_config.name, "hello");

    db::set_colorscheme(&conn, 1).expect(womp!());
    db::set_color(&conn, 0).expect(womp!());

    let db_config = db::get(&conn).expect(womp!());
    assert_eq!(db_config.color, 0);
    assert_eq!(db_config.colorscheme, 1);

    let test_picture = "test_resources/maryland.png";

    db::set_profile_picture(
        &conn,
        Some(image_utils::ProfilePicture::autocrop(
            test_picture.to_string(),
        )),
    )
    .expect(womp!("failed to set picture"));

    std::fs::remove_dir_all(pictures_dir()).expect(womp!());
}

#[test]
fn two_configs() {
    let mut conn = Database::in_memory().expect(womp!());
    let kp1 = KeyPair::gen_new();
    let id1 = "1".try_into().expect(womp!());
    let kp2 = KeyPair::gen_new();
    let id2 = "2".try_into().expect(womp!());

    ConfigBuilder::new(id1, kp1)
        .add_db(&mut conn)
        .expect(womp!());

    assert!(ConfigBuilder::new(id2, kp2).add_db(&mut conn).is_err());
}

#[test]
fn get_id() {
    let mut conn = Database::in_memory().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());
    let kp = KeyPair::gen_new();
    let config = ConfigBuilder::new(id, kp).add_db(&mut conn).expect(womp!());

    let db_id = db::id(&conn).expect(womp!());
    assert_eq!(config.id, db_id);
}

#[test]
fn get_kp() {
    let mut conn = Database::in_memory().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());
    let kp = KeyPair::gen_new();
    ConfigBuilder::new(id, kp.clone())
        .add_db(&mut conn)
        .expect(womp!());

    let keypair = db::keypair(&conn).expect(womp!());
    assert_eq!(keypair, kp);
}
