use coremacros::exit_err;
use herald_common::UserId;
use once_cell::sync::OnceCell;
use ratchet_chat::{protocol::ConversationStore, StoreLike};

pub struct Conn<'conn>(rusqlite::Transaction<'conn>);

impl StoreLike for Conn<'_> {
    type Error = rusqlite::Error;
}

impl ConversationStore for Conn<'_> {
    fn add_to_convo(
        &mut self,
        cid: ConversationId,
        members: Vec<UserId>,
    ) -> Result<(), Self::Error> {
    }

    fn left_convo(
        &mut self,
        cid: ConversationId,
        from: UserId,
    ) -> Result<(), Self::Error> {
    }

    fn get_members(
        &mut self,
        cid: ConversationId,
    ) -> Result<Vec<UserId>, Self::Error> {
    }

    fn member_of(
        &mut self,
        cid: ConversationId,
        uid: UserId,
    ) -> Result<bool, Self::Error> {
    }
}
