use super::*;
use crate::womp;
use herald_common::sig::KeyPair;
use serial_test_derive::serial;
use std::convert::TryInto;

#[test]
#[serial]
fn add_get_set_config() {
    Database::reset_all().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());

    let kp = KeyPair::gen_new();
    ConfigBuilder::new()
        .id(id)
        .keypair(kp)
        .add()
        .expect(womp!());

    let config = Config::get().expect(womp!());
    assert_eq!(config.id(), id);
    assert_eq!(config.colorscheme, 0);
    assert_eq!(config.color, crate::utils::id_to_color(id));
    assert_eq!(config.color, crate::utils::id_to_color(id));
    assert!(config.name.is_none());
    assert!(config.profile_picture.is_none());

    Database::reset_all().expect(womp!());

    let name = "stuff";
    let profile_picture = "stuff";
    let nts_id = [0u8; 32].into();
    let kp = KeyPair::gen_new();
    let config = ConfigBuilder::new()
        .id(id)
        .keypair(kp)
        .name(name.into())
        .colorscheme(1)
        .color(2)
        .profile_picture(profile_picture.into())
        .nts_conversation(nts_id)
        .add()
        .expect(womp!());

    let meta = crate::conversation::meta(&config.nts_conversation).expect(womp!());

    assert_eq!(meta.title.expect(womp!()), NTS_CONVERSATION_NAME);

    let db_config = Config::get().expect(womp!());

    assert_eq!(config.nts_conversation, db_config.nts_conversation);
    assert_eq!(db_config.id(), id);
    assert_eq!(db_config.name.as_ref().expect(womp!()), name);
    assert_eq!(db_config.nts_conversation, nts_id);
    assert_eq!(db_config.colorscheme, 1);
    assert_eq!(db_config.color, 2);
    assert_eq!(
        db_config.profile_picture.as_ref().expect(womp!()),
        profile_picture
    );

    let mut db_config = Config::get().expect(womp!());
    db_config.set_name(None).expect(womp!());
    assert_eq!(db_config.name, None);

    db_config.set_name(Some("hello".into())).expect(womp!());

    let mut db_config = Config::get().expect(womp!());
    assert_eq!(db_config.name, Some("hello".into()));

    db_config.set_colorscheme(1).expect(womp!());
    db_config.set_color(0).expect(womp!());

    let db_config = Config::get().expect(womp!());
    assert_eq!(db_config.color, 0);
    assert_eq!(db_config.colorscheme, 1);
}

#[test]
#[serial]
fn two_configs() {
    Database::reset_all().expect(womp!());
    let kp1 = KeyPair::gen_new();
    let id1 = "1".try_into().expect(womp!());
    let kp2 = KeyPair::gen_new();
    let id2 = "2".try_into().expect(womp!());

    ConfigBuilder::new()
        .id(id1)
        .keypair(kp1)
        .add()
        .expect(womp!());

    assert!(ConfigBuilder::new().id(id2).keypair(kp2).add().is_err());
}

#[test]
#[serial]
fn get_id() {
    Database::reset_all().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());
    let kp = KeyPair::gen_new();
    let config = ConfigBuilder::new()
        .id(id)
        .keypair(kp)
        .add()
        .expect(womp!());

    let static_id = Config::static_id().expect(womp!());
    assert_eq!(config.id, id);
    assert_eq!(config.id, static_id);
}

#[test]
#[serial]
fn get_kp() {
    Database::reset_all().expect(womp!());

    let id = "HelloWorld".try_into().expect(womp!());
    let kp = KeyPair::gen_new();
    let config = ConfigBuilder::new()
        .id(id)
        .keypair(kp.clone())
        .add()
        .expect(womp!());

    let static_keypair = Config::static_keypair().expect(womp!());
    assert_eq!(config.keypair, kp);
    assert_eq!(config.keypair, static_keypair);
}
