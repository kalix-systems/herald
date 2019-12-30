use super::*;
use coremacros::*;
use message_cache::cache;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

fn msg_constructor(
    id: MsgId,
    author: UserId,
) -> coretypes::messages::Message {
    let convid = [0; 32].into();

    coretypes::messages::Message {
        message_id: id,
        author,
        conversation: convid,
        content: Some(coretypes::messages::Item::Plain(
            "hello".try_into().expect(womp!()),
        )),

        time: coretypes::messages::MessageTime {
            server: None,
            insertion: Time::now(),
            expiration: None,
        },
        op: coretypes::messages::ReplyId::None,
        send_status: coretypes::messages::MessageSendStatus::Ack,
        receipts: HashMap::new(),

        replies: HashSet::new(),
        reactions: None,
        attachments: herald_attachments::AttachmentMeta::new(Vec::new()),
    }
}

#[test]

fn test_container() {
    let msgid1 = [0; 32].into();
    let msgid2 = [1; 32].into();
    let author = "author".try_into().expect(womp!());
    let msg1 = msg_constructor(msgid1, author);
    let msg2 = msg_constructor(msgid2, author);

    cache();

    let (msgmeta1, msgdata1) = coretypes::messages::split_msg(msg1);
    let (msgmeta2, msgdata2) = coretypes::messages::split_msg(msg2);
    let container = Container::new(vec![msgmeta1, msgmeta2], Some(msgdata2.clone()));

    cache::insert(msgid1, msgdata1);
    cache::insert(msgid2, msgdata2);

    assert_eq!(container.len(), 2);
}
