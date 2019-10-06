use crate::{
    ffi,
    interface::*,
    ret_err, ret_none,
    shared::{MsgUpdate, MSG_RXS},
};
use herald_common::UserId;
use heraldcore::{
    abort_err, chrono,
    config::Config,
    conversation,
    errors::HErr::{self, NoneError as NE},
    message::{self, Message as Msg},
    network,
    types::*,
};
use std::{collections::HashMap, convert::TryFrom, thread};

type Emitter = MessagesEmitter;
type List = MessagesList;

#[derive(Clone)]
/// A thin wrapper around a `MsgId`
pub struct Message {
    msg_id: MsgId,
}

/// A wrapper around a vector of `Message`s with additional fields
/// to facilitate interaction with QML.
pub struct Messages {
    emit: Emitter,
    model: List,
    list: Vec<Message>,
    map: HashMap<MsgId, Msg>,
    local_id: UserId,
    conversation_id: Option<ConversationId>,
    updated: chrono::DateTime<chrono::Utc>,
}

impl Messages {
    fn last_msg(&self) -> Option<&Msg> {
        let mid = self.list.last()?.msg_id;
        self.map.get(&mid)
    }

    fn emit_last_changed(&mut self) {
        self.emit.last_author_changed();
        self.emit.last_body_changed();
        self.emit.last_epoch_timestamp_ms_changed();
        self.emit.last_status_changed();
    }

    fn raw_insert(&mut self, body: String, op: Option<MsgId>) -> Result<(), HErr> {
        self.updated = chrono::Utc::now();

        let conversation_id = self.conversation_id.ok_or(NE)?;

        let (msg_id, timestamp) = message::add_message(
            None,
            self.local_id,
            &conversation_id,
            body.as_str(),
            None,
            None,
            &op,
        )?;

        let msg = Msg {
            author: self.local_id.clone(),
            body: (&body).clone(),
            conversation: conversation_id.clone(),
            message_id: msg_id.clone(),
            op,
            timestamp,
            receipts: None,
            send_status: MessageSendStatus::NoAck,
        };

        self.model
            .begin_insert_rows(self.row_count(), self.row_count());
        self.list.push(Message { msg_id });
        self.map.insert(msg_id, msg);
        self.model.end_insert_rows();

        self.emit_last_changed();

        thread::Builder::new().spawn(move || {
            // TODO update send status?
            ret_err!(network::send_text(conversation_id, body, msg_id, op));
        })?;

        Ok(())
    }
}

impl MessagesTrait for Messages {
    fn new(emit: Emitter, model: List) -> Self {
        Messages {
            list: Vec::new(),
            map: HashMap::new(),
            model,
            emit,
            conversation_id: None,
            local_id: abort_err!(Config::static_id()),
            updated: chrono::Utc::now(),
        }
    }

    fn last_author(&self) -> Option<ffi::UserIdRef> {
        let last = self.last_msg()?;

        if last.author == self.local_id {
            Some("You")
        } else {
            Some(last.author.as_str())
        }
    }

    fn last_status(&self) -> Option<u32> {
        self.last_msg()?
            .receipts
            .as_ref()?
            .iter()
            .map(|(_, status)| *status as u32)
            .max()
    }

    fn last_body(&self) -> Option<&str> {
        Some(self.last_msg()?.body.as_str())
    }

    fn last_epoch_timestamp_ms(&self) -> Option<i64> {
        Some(self.last_msg()?.timestamp.timestamp_millis())
    }

    fn set_conversation_id(&mut self, conversation_id: Option<ffi::ConversationIdRef>) {
        match (conversation_id, self.conversation_id) {
            (Some(id), None) => {
                let conversation_id = ret_err!(ConversationId::try_from(id));

                self.conversation_id = Some(conversation_id);
                self.emit.conversation_id_changed();

                let messages: Vec<Message> =
                    ret_err!(conversation::conversation_messages(&conversation_id))
                        .into_iter()
                        .map(|m| {
                            let mid = m.message_id;
                            self.map.insert(mid, m);
                            Message { msg_id: mid }
                        })
                        .collect();

                if messages.is_empty() {
                    return;
                }

                self.model
                    .begin_insert_rows(0, messages.len().saturating_sub(1));
                self.list = messages;
                self.model.end_insert_rows();
                self.emit_last_changed();
            }
            _ => {
                return;
            }
        }
    }

    fn conversation_id(&self) -> Option<ffi::ConversationIdRef> {
        self.conversation_id.as_ref().map(|c| c.as_slice())
    }

    fn author(&self, row_index: usize) -> ffi::UserIdRef {
        let mid = ret_none!(self.list.get(row_index), ffi::NULL_USER_ID).msg_id;
        ret_none!(self.map.get(&mid), ffi::NULL_USER_ID)
            .author
            .as_str()
    }

    fn body(&self, row_index: usize) -> &str {
        let mid = ret_none!(self.list.get(row_index), "").msg_id;
        ret_none!(self.map.get(&mid), "").body.as_str()
    }

    fn message_id(&self, row_index: usize) -> ffi::MsgIdRef {
        ret_none!(self.list.get(row_index), &ffi::NULL_MSG_ID)
            .msg_id
            .as_slice()
    }

    fn message_body_by_id(&self, msg_id: ffi::MsgIdRef) -> String {
        let msg_id = ret_err!(MsgId::try_from(msg_id), "".into());

        ret_none!(self.map.get(&msg_id), "".to_owned()).body.clone()
    }

    fn op(&self, row_index: usize) -> ffi::MsgIdRef {
        let mid = ret_none!(self.list.get(row_index), &ffi::NULL_MSG_ID).msg_id;

        match ret_none!(self.map.get(&mid), &ffi::NULL_MSG_ID).op.as_ref() {
            Some(op) => op.as_slice(),
            None => &ffi::NULL_MSG_ID,
        }
    }

    fn send_message(&mut self, body: String) -> bool {
        ret_err!(self.raw_insert(body, None), false);
        true
    }

    fn reply(&mut self, body: String, op: ffi::MsgIdRef) -> bool {
        let op = ret_err!(MsgId::try_from(op), false);

        ret_err!(self.raw_insert(body, Some(op)), false);
        true
    }

    fn delete_message(&mut self, row_index: u64) -> bool {
        let row_index = row_index as usize;

        let id = &ret_none!(self.list.get(row_index), false).msg_id;

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
        self.map = HashMap::new();
        self.model.end_reset_model();

        self.emit_last_changed();
        true
    }

    fn epoch_timestamp_ms(&self, row_index: usize) -> i64 {
        let mid = ret_none!(self.list.get(row_index), 0).msg_id;

        ret_none!(self.map.get(&mid), 0)
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

        let rx = match MSG_RXS.get(&conv_id) {
            Some(rx) => rx,
            // it's not a problem if the model doesn't have a receiver yet
            None => return true,
        };

        for update in rx.try_iter() {
            match update {
                MsgUpdate::Msg(mid) => {
                    // NOTE: temporary hack to avoid double insertions
                    //if self.map.contains_key(&mid) {
                    //    continue;
                    //}

                    let new = ret_err!(message::get_message(&mid), false);

                    self.updated = chrono::Utc::now();

                    self.model
                        .begin_insert_rows(self.list.len(), self.list.len());
                    self.list.push(Message {
                        msg_id: new.message_id,
                    });
                    self.map.insert(new.message_id, new);
                    self.model.end_insert_rows();

                    self.emit_last_changed();
                }
                MsgUpdate::Receipt { mid, by, stat } => {
                    let mut msg = match self.map.get_mut(&mid) {
                        None => {
                            // This can (possibly) happen if the message
                            // was deleted between the receipt
                            // being received over the network
                            // and this part of the code.
                            continue;
                        }
                        Some(msg) => msg,
                    };

                    // NOTE: If this fails, there is a bug somewhere
                    // in libherald.
                    //
                    // It is probably trivial, but may reflect something
                    // deeply wrong with our understanding of the program's
                    // concurrency.
                    let ix = ret_none!(
                        self.list
                            .iter()
                            // search backwards,
                            // it's probably fairly recent
                            .rposition(|m| m.msg_id == mid),
                        false
                    );

                    match &mut msg.receipts {
                        Some(receipts) => {
                            receipts.push((by, stat));
                        }
                        None => msg.receipts = Some(vec![(by, stat)]),
                    }

                    self.model.data_changed(ix, ix);
                }
            }
        }
        true
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }
}
