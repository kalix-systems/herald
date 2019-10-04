use crate::shared::{ConvUpdate, CONV_MSG_RXS};
use crate::{ffi, interface::*, ret_err, ret_none};
use herald_common::UserId;
use heraldcore::{
    abort_err, chrono,
    config::Config,
    conversation,
    message::{self, Message as Msg},
    types::*,
};
use std::convert::TryFrom;

type Emitter = MessagesEmitter;
type List = MessagesList;

#[derive(Clone)]
/// A thin wrapper around `heraldcore::message::Message`
pub struct Message {
    inner: Msg,
}

/// A wrapper around a vector of `Message`s with additional fields
/// to facilitate interaction with QML.
pub struct Messages {
    emit: Emitter,
    model: List,
    list: Vec<Message>,
    local_id: UserId,
    conversation_id: Option<ConversationId>,
    updated: chrono::DateTime<chrono::Utc>,
}

impl Messages {
    fn update_last(&mut self) {
        self.emit.last_author_changed();
        self.emit.last_body_changed();
        self.emit.last_epoch_timestamp_ms_changed();
        self.emit.last_status_changed();
    }

    fn raw_insert(&mut self, body: String, op: Option<MsgId>) -> Option<MsgId> {
        self.updated = chrono::Utc::now();

        let conversation_id = ret_none!(self.conversation_id, None);

        let (msg_id, timestamp) = ret_err!(
            message::add_message(
                None,
                self.local_id,
                &conversation_id,
                body.as_str(),
                None,
                None,
                &op
            ),
            None
        );

        let msg = Message {
            inner: Msg {
                author: self.local_id.clone(),
                body: body,
                conversation: conversation_id.clone(),
                message_id: msg_id.clone(),
                op,
                timestamp,
                receipts: None,
                send_status: MessageSendStatus::NoAck,
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
    fn new(emit: Emitter, model: List) -> Self {
        Messages {
            list: Vec::new(),
            model,
            emit,
            conversation_id: None,
            local_id: abort_err!(Config::static_id()),
            updated: chrono::Utc::now(),
        }
    }

    fn last_author(&self) -> Option<ffi::UserIdRef> {
        let inner = &self.list.last()?.inner;

        if inner.author == self.local_id {
            Some("You")
        } else {
            Some(inner.author.as_str())
        }
    }

    fn last_status(&self) -> Option<u32> {
        match self.list.last() {
            Some(msg) => {
                if let Some(status_vec) = &msg.inner.receipts {
                    status_vec.iter().map(|(_, status)| *status as u32).max()
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn last_body(&self) -> Option<&str> {
        match self.list.last() {
            Some(msg) => Some(msg.inner.body.as_str()),
            None => None,
        }
    }

    fn last_epoch_timestamp_ms(&self) -> Option<i64> {
        match self.list.last() {
            Some(msg) => Some(msg.inner.timestamp.timestamp_millis()),
            None => None,
        }
    }

    fn set_conversation_id(&mut self, conversation_id: Option<ffi::ConversationIdRef>) {
        match conversation_id {
            Some(id) => {
                let conversation_id = ret_err!(ConversationId::try_from(id));

                if self.conversation_id == Some(conversation_id) {
                    return;
                }

                self.conversation_id = Some(conversation_id);
                self.emit.conversation_id_changed();

                self.model.begin_reset_model();
                self.list = Vec::new();
                self.model.end_reset_model();

                let messages: Vec<Message> =
                    ret_err!(conversation::conversation_messages(&conversation_id))
                        .into_iter()
                        .map(|m| Message { inner: m })
                        .collect();

                if messages.is_empty() {
                    return;
                }

                self.model.begin_insert_rows(0, messages.len() - 1);
                self.list = messages;
                self.model.end_insert_rows();
                self.update_last();
            }
            None => {
                if self.conversation_id.is_none() {
                    return;
                }

                self.conversation_id = None;
                self.emit.conversation_id_changed();
                self.emit.conversation_id_changed();

                self.model.begin_reset_model();
                self.list = Vec::new();
                self.model.end_reset_model();
            }
        }
    }

    fn conversation_id(&self) -> Option<ffi::ConversationIdRef> {
        self.conversation_id.as_ref().map(|c| c.as_slice())
    }

    fn author(&self, row_index: usize) -> ffi::UserIdRef {
        ret_none!(self.list.get(row_index), "")
            .inner
            .author
            .as_str()
    }

    fn body(&self, row_index: usize) -> &str {
        ret_none!(self.list.get(row_index), "").inner.body.as_str()
    }

    fn message_id(&self, row_index: usize) -> ffi::MsgIdRef {
        ret_none!(self.list.get(row_index), &ffi::NULL_MSG_ID)
            .inner
            .message_id
            .as_slice()
    }

    fn message_body_by_id(&self, msg_id: ffi::MsgIdRef) -> String {
        let msg_id = ret_err!(MsgId::try_from(msg_id), "".into());

        self.list
            .iter()
            .find(|m| m.inner.message_id == msg_id)
            .map(|m| m.inner.body.clone())
            .unwrap_or("".into())
    }

    fn op(&self, row_index: usize) -> ffi::MsgIdRef {
        match &ret_none!(self.list.get(row_index), &ffi::NULL_MSG_ID)
            .inner
            .op
        {
            Some(id) => id.as_slice(),
            None => &ffi::NULL_MSG_ID,
        }
    }

    fn insert_message(&mut self, body: String) -> ffi::MsgId {
        match self.raw_insert(body, None) {
            Some(message_id) => {
                self.update_last();
                message_id.to_vec()
            }
            None => ffi::NULL_MSG_ID.to_vec(),
        }
    }

    fn reply(&mut self, body: String, op: ffi::MsgIdRef) -> ffi::MsgId {
        let op = ret_err!(MsgId::try_from(op), ffi::NULL_MSG_ID.to_vec());

        match self.raw_insert(body, Some(op)) {
            Some(message_id) => message_id.to_vec(),
            None => ffi::NULL_MSG_ID.to_vec(),
        }
    }

    fn delete_message(&mut self, row_index: u64) -> bool {
        let row_index = row_index as usize;
        let id = &self.list[row_index].inner.message_id;
        match message::delete_message(&id) {
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
    fn clear_conversation_history(&mut self) -> bool {
        let id = ret_none!(self.conversation_id, false);

        ret_err!(conversation::delete_conversation(&id), false);

        self.model.begin_reset_model();
        self.list = Vec::new();
        self.model.end_reset_model();
        true
    }

    fn epoch_timestamp_ms(&self, row_index: usize) -> i64 {
        ret_none!(self.list.get(row_index), 0)
            .inner
            .timestamp
            .timestamp_millis()
    }

    /// Clears the current view without modifying the underlying data
    fn clear_conversation_view(&mut self) {
        self.model.begin_reset_model();
        self.list = Vec::new();
        self.model.end_reset_model();
    }

    /// Polls for updates
    fn poll_update(&mut self) -> bool {
        let conv_id = ret_none!(self.conversation_id, true);

        let rx = match CONV_MSG_RXS.get(&conv_id) {
            Some(rx) => rx,
            None => return true,
        };

        let notif = ret_err!(rx.recv(), false);

        match notif {
            ConvUpdate::Msg(mid) => {
                let new = ret_err!(message::get_message(&mid), false);

                self.updated = chrono::Utc::now();

                self.model
                    .begin_insert_rows(self.list.len(), self.list.len());
                self.list.push(Message { inner: new });
                self.model.end_insert_rows();

                self.update_last();

                true
            }
            ConvUpdate::Ack(_mid) => {
                println!("TODO: Handle acks");
                true
            }
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }
}
