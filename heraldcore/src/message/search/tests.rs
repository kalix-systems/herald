use super::*;
use std::convert::TryInto;

#[test]
fn search() {
    let mut conn = Database::in_memory_with_config().expect(womp!());

    let receiver = crate::user::db::test_user(&mut conn, "receiver");

    let conv = receiver.pairwise_conversation;

    let mut builder = InboundMessageBuilder::default();
    let msg_id1 = [1; 32].into();
    builder
        .id(msg_id1)
        .author(receiver.id)
        .conversation_id(conv)
        .timestamp(Time::now())
        .body("search".try_into().expect(womp!()));
    builder.store_db(&mut conn).expect(womp!());

    let mut builder = InboundMessageBuilder::default();
    let msg_id2 = [2; 32].into();
    builder
        .id(msg_id2)
        .author(receiver.id)
        .conversation_id(conv)
        .timestamp(Time::now())
        .body("pattern".try_into().expect(womp!()));
    builder.store_db(&mut conn).expect(womp!());

    let mut builder = InboundMessageBuilder::default();
    let msg_id3 = [3; 32].into();
    builder
        .id(msg_id3)
        .author(receiver.id)
        .conversation_id(conv)
        .timestamp(Time::now())
        .body("patent".try_into().expect(womp!()));
    builder.store_db(&mut conn).expect(womp!());

    let pattern = SearchPattern::new_normal("pat".into()).expect(womp!());

    let mut searcher = Search::new(pattern);

    let first_page = searcher
        .next_page_db(&mut conn)
        // result
        .expect(womp!())
        // option
        .expect(womp!());

    assert_eq!(first_page.len(), 2);
    let (first, second) = (&first_page[0], &first_page[1]);

    assert_eq!(first_page[0].rowid, 3);
    assert_eq!(first_page[1].rowid, 2);
    assert_eq!(first_page[0].body.as_str(), "patent");
    assert_eq!(first_page[1].body.as_str(), "pattern");
    assert!(first.time >= second.time);

    let second_page = searcher.next_page_db(&mut conn).expect(womp!());
    assert!(second_page.is_none());
}
