use crate::{interface::*, ret_err, ret_none, types::*};
use herald_common::UserId;
use heraldcore::{
    abort_err, chrono,
    config::Config,
    message::{Message, Messages as Core},
    types::*,
};
use std::convert::TryFrom;

#[derive(Clone)]
struct MessagesItem {
    inner: Message,
}

pub struct Messages {
    conversation_id: Option<ConversationId>,
    emit: MessagesEmitter,
    model: MessagesList,
    list: Vec<MessagesItem>,
    handle: Core,
    local_id: UserId,
    updated: chrono::DateTime<chrono::Utc>,
}

impl Messages {
    fn raw_insert(&mut self, body: String, op: Option<MsgId>) -> Option<MsgId> {
        self.updated = chrono::Utc::now();

        let conversation_id = match &self.conversation_id {
            Some(conv) => conv,
            None => {
                eprintln!("Error: conversation_id not set.");
                return None;
            }
        };

        let (msg_id, timestamp) = ret_err!(
            self.handle.add_message(
                None,
                self.local_id,
                conversation_id,
                body.as_str(),
                None,
                &op
            ),
            None
        );

        let msg = MessagesItem {
            inner: Message {
                author: self.local_id,
                body: body,
                conversation: conversation_id.clone(),
                message_id: msg_id.clone(),
                op,
                timestamp,
                receipts: None,
                send_status: None,
            },
        };
        self.model
            .begin_insert_rows(self.row_count(), self.row_count());
        self.list.push(msg);
        self.model.end_insert_rows();
        Some(msg_id)
    }
}

impl MessagesTrait for Messages {
    fn new(emit: MessagesEmitter, model: MessagesList) -> Messages {
        Messages {
            conversation_id: None,
            list: Vec::new(),
            model,
            emit,
            local_id: abort_err!(Config::static_id()),
            handle: Core::new(),
            updated: chrono::Utc::now(),
        }
    }

    fn set_conversation_id(&mut self, conversation_id: Option<FfiConversationIdRef>) {
        let conversation_id = match conversation_id {
            Some(id) => Some(ret_err!(ConversationId::try_from(id))),
            None => None,
        };

        if self.conversation_id == conversation_id {
            return;
        }

        println!("Setting conversation_id to: {:?}", conversation_id);
        self.conversation_id = conversation_id;

        if let Some(conversation_id) = self.conversation_id.as_ref() {
            self.model.begin_reset_model();
            self.list = Vec::new();
            self.model.end_reset_model();

            let messages: Vec<MessagesItem> =
                ret_err!(self.handle.conversation_messages(&conversation_id))
                    .into_iter()
                    .map(|m| MessagesItem { inner: m })
                    .collect();

            if messages.is_empty() {
                return;
            }

            self.model.begin_insert_rows(0, messages.len() - 1);
            self.list = messages;
            self.model.end_insert_rows();
            self.emit.conversation_id_changed();
        }
    }

    fn author(&self, row_index: usize) -> FfiUserIdRef {
        ret_none!(self.list.get(row_index), "")
            .inner
            .author
            .as_str()
    }

    fn body(&self, row_index: usize) -> &str {
        ret_none!(self.list.get(row_index), "").inner.body.as_str()
    }

    fn message_id(&self, row_index: usize) -> FfiMsgIdRef {
        ret_none!(self.list.get(row_index), &[])
            .inner
            .message_id
            .as_slice()
    }

    fn op(&self, row_index: usize) -> Option<FfiMsgIdRef> {
        match &ret_none!(self.list.get(row_index), None).inner.op {
            Some(id) => Some(id.as_slice()),
            None => None,
        }
    }

    fn conversation_id(&self) -> Option<FfiConversationIdRef> {
        match &self.conversation_id {
            Some(id) => Some(id.as_slice()),
            None => None,
        }
    }

    fn insert_message(&mut self, body: String) -> FfiMsgId {
        match self.raw_insert(body, None) {
            Some(message_id) => message_id.to_vec(),
            None => vec![],
        }
    }

    fn reply(&mut self, body: String, op: FfiMsgIdRef) -> FfiMsgId {
        let op = match MsgId::try_from(op) {
            Ok(op) => op,
            Err(e) => {
                eprintln!("{}", e);
                return vec![];
            }
        };

        match self.raw_insert(body, Some(op)) {
            Some(message_id) => message_id.to_vec(),
            None => vec![],
        }
    }

    fn delete_message(&mut self, row_index: u64) -> bool {
        let row_index = row_index as usize;
        let id = &self.list[row_index].inner.message_id;
        match self.handle.delete_message(&id) {
            Ok(_) => {
                self.model.begin_remove_rows(row_index, row_index);
                self.list.remove(row_index);
                self.model.end_remove_rows();
                true
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                false
            }
        }
    }

    /// Deletes all messages in the current conversation.
    fn delete_conversation(&mut self) -> bool {
        let id = match &self.conversation_id {
            Some(id) => id,
            None => {
                eprintln!("Warning: Conversation id not set");
                return false;
            }
        };

        ret_err!(self.handle.delete_conversation(id), false);

        self.model.begin_reset_model();
        self.list = Vec::new();
        self.conversation_id = None;
        self.model.end_reset_model();
        true
    }

    fn epoch_timestamp_ms(&self, row_index: usize) -> i64 {
        ret_none!(self.list.get(row_index), 0)
            .inner
            .timestamp
            .timestamp_millis()
    }

    /// Deletes all messages in a conversation.
    fn delete_conversation_by_id(&mut self, id: FfiConversationIdRef) -> bool {
        let id = ret_err!(ConversationId::try_from(id), false);

        ret_err!(self.handle.delete_conversation(&id), false);

        if Some(id) == self.conversation_id {
            self.model.begin_reset_model();
            self.list = Vec::new();
            self.model.end_reset_model();
        }

        true
    }

    /// Clears the current view without modifying the underlying data
    fn clear_conversation_view(&mut self) {
        self.model.begin_reset_model();
        self.list = Vec::new();
        self.conversation_id = None;
        self.model.end_reset_model();
    }

    fn refresh(&mut self) -> bool {
        let conv_id = match self.conversation_id {
            Some(id) => id,
            None => {
                return true;
            }
        };

        let new = ret_err!(
            self.handle
                .conversation_messages_since(&conv_id, self.updated),
            false
        );

        self.updated = chrono::Utc::now();

        if new.is_empty() {
            return true;
        }

        self.model.begin_insert_rows(
            self.list.len(),
            (self.list.len() + new.len()).saturating_sub(1),
        );
        self.list
            .extend(new.into_iter().map(|inner| MessagesItem { inner }));
        self.model.end_insert_rows();

        true
    }

    fn emit(&mut self) -> &mut MessagesEmitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }
}
