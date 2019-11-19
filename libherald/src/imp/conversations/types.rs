use super::*;
use herald_common::Time;

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
        pairwise,
        last_active,
        expiration_period,
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
            pairwise,
            last_active,
            expiration_period,
        },
    )
}

pub(super) struct Data {
    /// Conversation title
    pub title: Option<String>,
    /// Conversation picture
    pub picture: Option<String>,
    /// Conversation color,
    pub color: u32,
    /// Indicates whether the conversation is muted
    pub muted: bool,
    /// Indicates whether the conversation is a canonical pairwise conversation
    pub pairwise: bool,
    /// Last notable activity
    pub last_active: Time,
    /// Time until message expiration
    pub expiration_period: ExpirationPeriod,
}
