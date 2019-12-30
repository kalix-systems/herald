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

    builder.store().expect(womp!()).split()
}

#[serial]
#[test]
fn test_insertion_flurry_deletion() {
    heraldcore::db::reset_all().expect(womp!());

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

    assert_eq!(container.same_flurry(0, 1).expect(womp!()), true);

    let ix1 = container.index_of(&msgmeta1);
    assert!(ix1.is_some());
    assert_eq!(ix1, Some(1));

    let ix2 = container.index_of(&msgmeta2);
    assert!(ix2.is_some());
    assert_eq!(ix2, Some(0));
}

#[serial]
#[test]
fn test_container_search() {
    heraldcore::db::reset_all().expect(womp!());

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
    let (msgmeta2, msgdata2) = msg_constructor("retesting");
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
    searchstate.active = true;

    let matches = container
        .apply_search(&searchstate, |_| (), || ())
        .expect(womp!("Failed to apply search"));

    searchstate.set_matches(matches, || (), || ());

    assert_eq!(searchstate.num_matches(), 2);

    assert_eq!(
        container.get(0).expect(womp!()).match_status,
        MatchStatus::NotMatched
    );
    assert_eq!(
        container.get(1).expect(womp!()).match_status,
        MatchStatus::Matched
    );

    assert_eq!(
        &searchstate.next_match().expect(womp!()).0,
        container.get(2).expect(womp!())
    );

    assert_eq!(
        &searchstate.prev_match().expect(womp!()).0,
        container.get(1).expect(womp!())
    );

    container.clear_search(|_| ());

    assert_eq!(msgmeta1.match_status, MatchStatus::NotMatched);

    assert_eq!(msgmeta2.match_status, MatchStatus::NotMatched);

    assert_eq!(msgmeta3.match_status, MatchStatus::NotMatched);
}
