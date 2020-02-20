use super::*;
use herald_common::*;
use herald_ids::*;

#[derive(Debug)]
pub enum NetworkAction {
    UpdateProfile(cmessages::ProfileChanged),
    StartConvo(Box<cmessages::AddedToConvo>),
    UpdateSettings {
        mid: MsgId,
        cid: ConversationId,
        expiration: Option<Time>,
        update: cmessages::GroupSettingsUpdate,
    },
    // Receipt {
    //     cid: ConversationId,
    //     msg_id: MsgId,
    // },
    // TypingIn(ConversationId),
    // UserReq {
    //     uid: UserId,
    //     cid: ConversationId,
    // },
    // Message {
    //     cid: ConversationId,
    //     msg: cmessages::Msg,
    // },
    // React {
    //     cid: ConversationId,
    //     msg_id: MsgId,
    //     body: String,
    // },
    // RemoveReact {
    //     cid: ConversationId,
    //     msg_id: MsgId,
    //     body: String,
    // },
}
