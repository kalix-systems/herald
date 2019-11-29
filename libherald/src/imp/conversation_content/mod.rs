use crate::{
    ffi,
    imp::{Members, Messages},
    interface::{ConversationContentEmitter as Emitter, ConversationContentTrait as Interface},
    ret_err,
};
use heraldcore::types::ConversationId;
use std::convert::TryFrom;

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

            self.messages.set_conversation_id(id);
            ret_err!(self.members.set_conversation_id(id));

            self.id.replace(id);
            self.emit.conversation_id_changed();
        }
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
}
