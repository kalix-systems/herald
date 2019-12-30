use super::*;
use coremacros::*;
use serial_test_derive::serial;
use std::convert::TryInto;

fn msg_constructor(body: &str) -> (MessageMeta, MsgData) {
    let convid = [0; 32].into();
    let mut builder = heraldcore::message::OutboundMessageBuilder::default();

    builder
        .body(body.try_into().expect(womp!()))
        .conversation_id(convid);

    coretypes::messages::split_msg(builder.store().expect(womp!()))
}

#[test]
#[serial]
fn test_container() {
    drop(std::fs::remove_dir_all(".data_dir"));
    heraldcore::db::init().expect(womp!("Failed to initialize database"));

    let convid = [0; 32].into();

    heraldcore::config::ConfigBuilder::new(
        "TEST".try_into().expect(womp!()),
        herald_common::sig::KeyPair::gen_new(),
    )
    .nts_conversation(convid)
    .add()
    .expect(womp!("Failed to add config"));

    let (msgmeta1, msgdata1) = msg_constructor("hello");
    std::thread::sleep(std::time::Duration::from_millis(2));
    let (msgmeta2, msgdata2) = msg_constructor("hello");
    let mut container = Container::new(vec![], None);

    let _ = container.insert_ord(msgmeta1, msgdata1);
    let _ = container.insert_ord(msgmeta2, msgdata2);

    assert_ne!(msgmeta1, msgmeta2);

    assert_eq!(container.len(), 2);

    let ix1 = container.index_of(&msgmeta1);
    assert!(ix1.is_some());
    assert_eq!(ix1, Some(1));

    let ix2 = container.index_of(&msgmeta2);
    assert!(ix2.is_some());
    assert_eq!(ix2, Some(0));
}

#[test]
#[serial]
fn test_container_search() {
    drop(std::fs::remove_dir_all(".data_dir"));

    heraldcore::db::init().expect(womp!("Failed to initialize database"));

    let convid = [0; 32].into();

    heraldcore::config::ConfigBuilder::new(
        "TEST".try_into().expect(womp!()),
        herald_common::sig::KeyPair::gen_new(),
    )
    .nts_conversation(convid)
    .add()
    .expect(womp!("Failed to add config"));

    let (msgmeta1, msgdata1) = msg_constructor("test");
    std::thread::sleep(std::time::Duration::from_millis(2));
    let (msgmeta2, msgdata2) = msg_constructor("testing");
    std::thread::sleep(std::time::Duration::from_millis(2));
    let (msgmeta3, msgdata3) = msg_constructor("tost");
    let mut container = Container::new(vec![], None);

    let _ = container.insert_ord(msgmeta1, msgdata1);
    let _ = container.insert_ord(msgmeta2, msgdata2);
    let _ = container.insert_ord(msgmeta3, msgdata3);

    let mut searchstate = SearchState::new();

    searchstate
        .set_pattern("te".to_string(), || ())
        .expect(womp!());

    // let matches = container
    //   .apply_search(&searchstate, |_i| (), || ())
    //     .expect(womp!("Failed to apply search"));

    //assert_eq!(matches.len(), 2);
}
