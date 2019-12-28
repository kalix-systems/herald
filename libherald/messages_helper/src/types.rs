pub use heraldcore::{message::*, types::MsgId};

pub fn from_msg_id(msg_id: MsgId) -> Option<MessageMeta> {
    let insertion_time = crate::container::access(&msg_id, |m| m.time.insertion)?;

    Some(MessageMeta {
        msg_id,
        insertion_time,
        match_status: MatchStatus::NotMatched,
    })
}
