use super::*;
use coremacros::*;
use serial_test_derive::serial;
use std::convert::TryInto;

pub struct TestModel {
    data_changed_state: Vec<(usize, usize)>,
}

impl TestModel {
    fn new() -> TestModel {
        TestModel {
            data_changed_state: Vec::new(),
        }
    }
}

impl MessageModel for TestModel {
    fn data_changed(
        &mut self,
        a: usize,
        b: usize,
    ) {
        self.data_changed_state.push((a, b));
    }
}

pub struct TestEmit {
    num_matches_state: u32,
    pattern_changed_state: u32,
    regex_changed_state: u32,
    index_changed_state: u32,
}

impl TestEmit {
    fn new() -> TestEmit {
        TestEmit {
            num_matches_state: 0,
            pattern_changed_state: 0,
            regex_changed_state: 0,
            index_changed_state: 0,
        }
    }
}

impl MessageEmit for TestEmit {
    fn search_index_changed(&mut self) {
        self.index_changed_state += 1;
    }
    fn search_num_matches_changed(&mut self) {
        self.num_matches_state += 1;
    }

    fn search_pattern_changed(&mut self) {
        self.pattern_changed_state += 1;
    }
    fn search_regex_changed(&mut self) {
        self.regex_changed_state += 1;
    }
    fn last_has_attachments_changed(&mut self) {}
}

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

    let mut container = Container::new(vec![]);

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

    let mut container = Container::new(vec![]);

    let _ = container.insert_ord(msgmeta1, msgdata1);
    let _ = container.insert_ord(msgmeta2, msgdata2);
    let _ = container.insert_ord(msgmeta3, msgdata3);

    let mut searchstate = SearchState::new();

    let emit = &mut TestEmit::new();
    let model = &mut TestModel::new();
    searchstate
        .set_pattern("te".to_string(), emit)
        .expect(womp!());
    searchstate.active = true;

    assert_eq!(emit.pattern_changed_state, 1);

    let matches = container
        .apply_search(&searchstate, emit, model)
        .expect(womp!("Failed to apply search"));

    assert_eq!(emit.num_matches_state, 1);

    searchstate.set_matches(matches, emit);

    assert_eq!(emit.num_matches_state, 2);

    assert_eq!(emit.index_changed_state, 1);

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

    container.clear_search(model);

    assert_eq!(msgmeta1.match_status, MatchStatus::NotMatched);

    assert_eq!(msgmeta2.match_status, MatchStatus::NotMatched);

    assert_eq!(msgmeta3.match_status, MatchStatus::NotMatched);

    assert_eq!(model.data_changed_state[0], (1, 1));
    assert_eq!(model.data_changed_state[1], (2, 2));
}

#[test]
#[serial]
fn test_handle_receipt() {
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
    let (msgmeta2, msgdata2) = msg_constructor("123");

    let mut container = Container::new(vec![]);

    let _ = container.insert_ord(msgmeta1, msgdata1);
    let _ = container.insert_ord(msgmeta2, msgdata2);

    let model = &mut TestModel::new();

    container.handle_receipt(
        msgmeta2.msg_id,
        coretypes::messages::MessageReceiptStatus::Read,
        "TEST".try_into().expect(womp!()),
        model,
    );

    assert_eq!(model.data_changed_state.len(), 1);
    assert_eq!(model.data_changed_state[0], (0, 0));
    assert_eq!(
        container
            .get_data(&msgmeta2.msg_id)
            .expect(womp!())
            .receipts[&"TEST".try_into().expect(womp!())],
        coretypes::messages::MessageReceiptStatus::Read
    );
}
