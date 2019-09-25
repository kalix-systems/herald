use crate::{interface::*, ret_err, ret_none, types::*};
use herald_common::UserIdRef;
use heraldcore::{
    abort_err, chrono,
    config::Config,
    conversation::{self, ConversationMeta},
    message::{self, Message as Msg},
    types::*,
};
use std::convert::TryFrom;

#[derive(Clone)]
struct Message {
    inner: Msg,
}

pub struct Conversation {
    emit: ConversationEmitter,
    model: ConversationList,
    list: Vec<Message>,
    meta: Option<ConversationMeta>,
    local_id: String,
    updated: chrono::DateTime<chrono::Utc>,
}

impl Conversation {
    fn current_cid(&self) -> Option<ConversationId> {
        Some(self.meta.as_ref()?.conversation_id)
    }

    fn raw_insert(&mut self, body: String, op: Option<MsgId>) -> Option<MsgId> {
        self.updated = chrono::Utc::now();

        let conversation_id = ret_none!(&self.meta, None).conversation_id;

        let (msg_id, timestamp) = ret_err!(
            message::add_message(
                None,
                self.local_id.as_str(),
                &conversation_id,
                body.as_str(),
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

impl ConversationTrait for Conversation {
    fn new(emit: ConversationEmitter, model: ConversationList) -> Self {
        Conversation {
            list: Vec::new(),
            model,
            meta: None,
            emit,
            local_id: abort_err!(Config::static_id()),
            updated: chrono::Utc::now(),
        }
    }

    fn pairwise(&self) -> bool {
        ret_none!(&self.meta, false).pairwise
    }

    fn color(&self) -> u32 {
        ret_none!(&self.meta, 0).color
    }

    fn set_color(&mut self, color: u32) {
        let meta = ret_none!(&mut self.meta);
        ret_err!(conversation::set_color(&meta.conversation_id, color));

        meta.color = color;
        self.emit.color_changed();
    }

    fn muted(&self) -> bool {
        ret_none!(&self.meta, false).muted
    }

    fn set_muted(&mut self, muted: bool) {
        let meta = ret_none!(&mut self.meta);
        ret_err!(conversation::set_muted(&meta.conversation_id, muted));

        meta.muted = muted;
        self.emit.muted_changed();
    }

    fn title(&self) -> Option<&str> {
        ret_none!(&self.meta, None)
            .title
            .as_ref()
            .map(|t| t.as_str())
    }

    fn set_title(&mut self, title: Option<String>) {
        let meta = ret_none!(&mut self.meta);

        ret_err!(conversation::set_title(
            &meta.conversation_id,
            title.as_ref().map(|t| t.as_str())
        ));

        meta.title = title;
    }

    fn picture(&self) -> Option<&str> {
        // Note: this should not be using the `?` operator
        ret_none!(&self.meta, None)
            .picture
            .as_ref()
            .map(|p| p.as_str())
    }

    fn set_picture(&mut self, picture: Option<String>) {
        let meta = &mut ret_none!(&mut self.meta);

        ret_err!(conversation::set_picture(
            &meta.conversation_id,
            picture.as_ref().map(|p| p.as_str()),
            meta.picture.as_ref().map(|p| p.as_str())
        ));

        meta.picture = picture;
        self.emit.picture_changed();
    }

    fn set_conversation_id(&mut self, conversation_id: Option<FfiConversationIdRef>) {
        match conversation_id {
            Some(id) => {
                let conversation_id = ret_err!(ConversationId::try_from(id));

                if self.current_cid() == Some(conversation_id) {
                    return;
                }

                self.meta = Some(ret_err!(conversation::meta(&conversation_id)));
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
            }
            None => {
                if self.meta.is_none() {
                    return;
                }
                self.emit.conversation_id_changed();

                self.model.begin_reset_model();
                self.list = Vec::new();
                self.model.end_reset_model();
            }
        }
    }

    fn author(&self, row_index: usize) -> UserIdRef {
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
        match &self.meta {
            Some(meta) => Some(meta.conversation_id.as_slice()),
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
        let op = ret_err!(MsgId::try_from(op), vec![]);

        match self.raw_insert(body, Some(op)) {
            Some(message_id) => message_id.to_vec(),
            None => vec![],
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
    fn delete_conversation(&mut self) -> bool {
        let id = match self.meta.as_mut() {
            Some(meta) => meta.conversation_id,
            None => {
                eprintln!("Warning: Conversation id not set");
                return false;
            }
        };

        ret_err!(conversation::delete_conversation(&id), false);

        self.model.begin_reset_model();
        self.list = Vec::new();
        self.meta = None;
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

        ret_err!(conversation::delete_conversation(&id), false);

        // TODO: delete this API?
        //if Some(id) == self.conversation_id {
        //    self.model.begin_reset_model();
        //    self.list = Vec::new();
        //    self.model.end_reset_model();
        //}

        true
    }

    /// Clears the current view without modifying the underlying data
    fn clear_conversation_view(&mut self) {
        self.model.begin_reset_model();
        self.list = Vec::new();
        self.model.end_reset_model();
    }

    fn refresh(&mut self) -> bool {
        let conv_id = match self.meta.as_mut() {
            Some(meta) => meta.conversation_id,
            None => {
                return true;
            }
        };

        let new = ret_err!(
            conversation::conversation_messages_since(&conv_id, self.updated),
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
            .extend(new.into_iter().map(|inner| Message { inner }));
        self.model.end_insert_rows();

        true
    }

    fn emit(&mut self) -> &mut ConversationEmitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }
}
