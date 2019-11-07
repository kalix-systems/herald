use super::*;
use crate::{conversation::db::set_expiration_period, message, types::ExpirationPeriod, womp};
use std::convert::TryInto;

#[test]
fn get_and_delete_stale() {
    let mut conn = Database::in_memory().expect(womp!());

    let receiver = crate::contact::db::test_contact(&mut conn, "receiver");
    crate::contact::db::test_contact(&mut conn, "dummy");

    let conv = receiver.pairwise_conversation;

    set_expiration_period(&conn, &conv, ExpirationPeriod::OneMinute).expect(womp!());

    let mut builder = InboundMessageBuilder::default();
    let msg_id = [0; 32].into();
    builder
        .id(msg_id)
        .author(receiver.id)
        .conversation_id(conv)
        .expiration(Time(
            Time::now().0 - Duration::from_secs(120).as_millis() as i64,
        ))
        .timestamp(Time::now())
        .body("hi".try_into().expect(womp!()));

    builder.store_db(&mut conn).expect(womp!());

    let msg = message::db::get_message(&conn, &msg_id).expect(womp!("unable to get message"));
    assert!(msg.time.expiration.unwrap() < Time::now());

    let stale = db::get_stale_conversations(&conn).expect(womp!());

    assert_eq!(stale.len(), 1);
    assert_eq!(stale.get(&conv).expect(womp!()), &[msg_id]);

    db::delete_expired(&conn).expect(womp!());

    assert! {
        message::db::get_message(&conn, &msg_id).is_err()
    };
}
