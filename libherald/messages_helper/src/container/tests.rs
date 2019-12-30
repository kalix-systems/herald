use super::*;
use coremacros::*;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use types::*;

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
    let author = "author".try_into().expect(womp!());
    let msg1 = msg_constructor([0; 32].into(), author);
    let msg2 = msg_constructor([1; 32].into(), author);
}
