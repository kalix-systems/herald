use super::*;
use crate::{
    err, ffi,
    interface::{MessagesEmitter as Emitter, MessagesList as List, MessagesTrait as Interface},
    none, spawn,
};
use heraldcore::{
    config,
    message::{self, MessageReceiptStatus, MsgData},
};
use messages_helper::search::SearchState;
use std::collections::HashMap;
use std::convert::TryInto;

mod attachments;
mod author;
mod body;
mod flurry;
mod last;
mod op;
mod reactions;
mod receipts;
mod search;
mod time;

impl Messages {
    pub(crate) fn new_(
        emit: Emitter,
        model: List,
        builder: MessageBuilder,
    ) -> Self {
        Messages {
            model,
            emit,
            container: Container::default(),
            conversation_id: None,
            local_id: config::id().ok(),
            search: SearchState::new(),
            builder,
            elider: Default::default(),
        }
    }

    pub(crate) fn msg_id_(
        &self,
        index: usize,
    ) -> Option<ffi::MsgIdRef> {
        Some(self.container.msg_id(index)?.as_slice())
    }

    pub(crate) fn index_by_id_(
        &self,
        msg_id: ffi::MsgIdRef,
    ) -> i64 {
        let msg_id = err!(msg_id.try_into(), -1);

        none!(self.container.index_by_id(msg_id), -1) as i64
    }

    pub(crate) fn delete_message_(
        &mut self,
        index: u64,
    ) -> bool {
        let ix = index as usize;

        let id = none!(self.container.get(ix), false).msg_id;

        self.remove_helper(id, ix);
        spawn!(message::delete_message(&id), false);

        true
    }

    pub(crate) fn clear_conversation_history_(&mut self) -> bool {
        let id = none!(self.conversation_id, false);

        spawn!(message::delete_conversation_messages(&id), false);

        self.clear_search();
        self.model
            .begin_remove_rows(0, self.container.len().saturating_sub(1));
        self.container = Default::default();
        self.model.end_remove_rows();

        self.emit_last_changed();
        self.emit.is_empty_changed();
        true
    }

    pub(crate) fn aux_data_(
        &self,
        index: usize,
    ) -> String {
        self.container.aux_data_json(index).unwrap_or_default()
    }

    pub(crate) fn is_empty_(&self) -> bool {
        self.container.is_empty()
    }

    pub(crate) fn emit_(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    pub(crate) fn row_count_(&self) -> usize {
        self.container.len()
    }

    pub(crate) fn builder_(&self) -> &MessageBuilder {
        &self.builder
    }

    pub(crate) fn builder_mut_(&mut self) -> &mut MessageBuilder {
        &mut self.builder
    }
}
