use super::*;
use herald_common::{Time, UserId};

/// Thin wrapper around `ConversationId`,
/// with an additional field to facilitate filtering
/// in the UI.
#[derive(Copy, Clone, Debug)]
pub struct Conversation {
    pub(super) id: ConversationId,
    pub(super) matched: bool,
}

pub(super) fn split_meta(meta: ConversationMeta) -> (Conversation, Data) {
    let ConversationMeta {
        title,
        conversation_id,
        picture,
        muted,
        color,
        pairwise_uid,
        last_active,
        expiration_period,
        status,
        last_msg_id,
    } = meta;

    (
        Conversation {
            id: conversation_id,
            matched: true,
        },
        Data {
            title,
            picture,
            color,
            muted,
            pairwise_uid,
            last_active,
            expiration_period,
            status,
            last_msg_id,
        },
    )
}

pub(crate) struct Data {
    /// Conversation title
    pub title: Option<String>,
    /// Conversation picture
    pub picture: Option<String>,
    /// Conversation color,
    pub color: u32,
    /// Indicates whether the conversation is muted
    pub muted: bool,
    /// Associated user id if the conversation is a canonical pairwise conversation
    pub pairwise_uid: Option<UserId>,
    /// Last notable activity
    pub last_active: Time,
    /// Time until message expiration
    pub expiration_period: ExpirationPeriod,
    /// Conversation archive status
    pub status: heraldcore::conversation::Status,
    /// Last message id
    pub last_msg_id: Option<heraldcore::types::MsgId>,
}
