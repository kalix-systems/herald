use crate::{
    ffi,
    imp::{Members, Messages},
    interface::{
        ConversationContentEmitter as Emitter, ConversationContentList as List,
        ConversationContentTrait as Interface,
    },
    members::MemberUpdate,
    ret_err, ret_none,
};
use heraldcore::types::ConversationId;
use std::convert::TryFrom;

mod shared;
pub(crate) use shared::content_push;

/// Wrapper around `Messages` and `Members`
pub struct ConversationContent {
    emit: Emitter,
    members: Members,
    messages: Messages,
    id: Option<ConversationId>,
}

impl Interface for ConversationContent {
    fn new(
        emit: Emitter,
        _: List,
        members: Members,
        messages: Messages,
    ) -> Self {
        Self {
            emit,
            members,
            messages,
            id: None,
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn conversation_id(&self) -> Option<ffi::ConversationIdRef> {
        self.id.as_ref().map(ConversationId::as_slice)
    }

    fn set_conversation_id(
        &mut self,
        cid: Option<ffi::ConversationIdRef>,
    ) {
        if let (Some(id), None) = (cid, self.id) {
            let id = ret_err!(ConversationId::try_from(id));

            self.id.replace(id);
            self.emit.conversation_id_changed();
            ret_none!(self.register_model());

            self.messages.set_conversation_id(id);
            ret_err!(self.members.set_conversation_id(id));
        }
    }

    fn can_fetch_more(&self) -> bool {
        self.id.as_ref().map(shared::more_updates).unwrap_or(false)
    }

    fn fetch_more(&mut self) {
        ret_none!(self.process_updates());
    }

    fn members(&self) -> &Members {
        &self.members
    }

    fn members_mut(&mut self) -> &mut Members {
        &mut self.members
    }

    fn messages(&self) -> &Messages {
        &self.messages
    }

    fn messages_mut(&mut self) -> &mut Messages {
        &mut self.messages
    }

    fn row_count(&self) -> usize {
        0
    }
}
