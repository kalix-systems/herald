use super::*;
use coremacros::*;
use std::convert::TryInto;

fn msg_constructor() -> (MessageMeta, MsgData) {
    let convid = [0; 32].into();
    let mut builder = heraldcore::message::OutboundMessageBuilder::default();

    builder
        .body("hello".try_into().expect(womp!()))
        .conversation_id(convid);

    coretypes::messages::split_msg(builder.store().expect(womp!()))
}
#[test]
fn test_container() {
    heraldcore::db::init().expect(womp!("Failed to initialize database"));
    let convid = [0; 32].into();

    heraldcore::config::ConfigBuilder::new(
        "TEST".try_into().expect(womp!()),
        herald_common::sig::KeyPair::gen_new(),
    )
    .nts_conversation(convid)
    .add()
    .expect(womp!("Failed to add config"));
    let (msgmeta1, msgdata1) = msg_constructor();
    let (msgmeta2, msgdata2) = msg_constructor();
    let container = Container::new(vec![msgmeta1, msgmeta2], Some(msgdata2.clone()));

    assert_eq!(container.len(), 2);

    std::fs::remove_dir_all(".data_dir").expect(womp!());
}
