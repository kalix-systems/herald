use crate::{ffi, interface::*, ret_err, ret_none, shared::messages::*, toasts::new_msg_toast};
use herald_common::UserId;
use heraldcore::{
    abort_err,
    config::Config,
    conversation,
    errors::HErr,
    message::{self, Message as Msg},
    types::*,
    NE,
};
use std::ops::Drop;
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

type Emitter = MessagesEmitter;
type List = MessagesList;

#[derive(Clone)]
/// A thin wrapper around a `MsgId`
pub struct Message {
    msg_id: MsgId,
    data_saved: bool,
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

    fn raw_insert(&mut self, msg: Msg, data_saved: bool) -> Result<(), HErr> {
        let msg_id = msg.message_id;
        let cid = self.conversation_id.ok_or(NE!())?;
        self.model
            .begin_insert_rows(self.row_count(), self.row_count());
        self.list.push(Message { msg_id, data_saved });
        self.map.insert(msg_id, msg);
        self.model.end_insert_rows();

        self.emit_last_changed();

        use crate::implementation::conversations::shared::*;
        // this should not return an error
        ret_none!(push_conv_update(ConvUpdates::NewActivity(cid)), Ok(()));

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
            .iter()
            .map(|(_, status)| *status as u32)
            .max()
    }

    fn last_body(&self) -> Option<&str> {
        Some(self.last_msg()?.body.as_ref()?.as_str())
    }

    fn last_epoch_timestamp_ms(&self) -> Option<i64> {
        Some(self.last_msg()?.timestamp.timestamp_millis())
    }

    /// Returns index of a message given its id.
    fn index_by_id(&self, msg_id: ffi::MsgIdRef) -> u64 {
        let msg_id = ret_err!(msg_id.try_into(), std::u32::MAX as u64);

        // sanity check
        if !self.map.contains_key(&msg_id) {
            return std::u64::MAX;
        }

        // search backwards
        self.list
            .iter()
            .rposition(|mid| msg_id == mid.msg_id)
            .map(|ix| ix as u64)
            .unwrap_or(std::u32::MAX as u64)
    }

    fn set_conversation_id(&mut self, conversation_id: Option<ffi::ConversationIdRef>) {
        match (conversation_id, self.conversation_id) {
            (Some(id), None) => {
                let conversation_id = ret_err!(ConversationId::try_from(id));

                MSG_EMITTERS.insert(conversation_id, self.emit().clone());

                self.conversation_id = Some(conversation_id);
                self.emit.conversation_id_changed();

                let messages: Vec<Message> =
                    ret_err!(conversation::conversation_messages(&conversation_id))
                        .into_iter()
                        .map(|m| {
                            let mid = m.message_id;
                            self.map.insert(mid, m);
                            Message {
                                msg_id: mid,
                                data_saved: true,
                            }
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

    fn data_saved(&self, row_index: usize) -> bool {
        ret_none!(self.list.get(row_index), false).data_saved
    }

    fn author(&self, row_index: usize) -> ffi::UserIdRef {
        let mid = ret_none!(self.list.get(row_index), ffi::NULL_USER_ID).msg_id;
        ret_none!(self.map.get(&mid), ffi::NULL_USER_ID)
            .author
            .as_str()
    }

    fn body(&self, row_index: usize) -> Option<&str> {
        let mid = self.list.get(row_index)?.msg_id;
        Some(self.map.get(&mid)?.body.as_ref()?.as_str())
    }

    fn message_id(&self, row_index: usize) -> ffi::MsgIdRef {
        ret_none!(self.list.get(row_index), &ffi::NULL_MSG_ID)
            .msg_id
            .as_slice()
    }

    fn has_attachments(&self, row_index: usize) -> bool {
        let mid = ret_none!(self.list.get(row_index), false).msg_id;
        ret_none!(self.map.get(&mid), false).has_attachments
    }

    fn receipt_status(&self, row_index: usize) -> u32 {
        let mid = ret_none!(self.list.get(row_index), MessageReceiptStatus::NoAck as u32).msg_id;
        let inner = ret_none!(self.map.get(&mid), MessageReceiptStatus::NoAck as u32);

        inner
            .receipts
            .values()
            .map(|r| *r as u32)
            .max()
            .unwrap_or(MessageReceiptStatus::NoAck as u32)
    }

    fn message_body_by_id(&self, msg_id: ffi::MsgIdRef) -> String {
        let msg_id = ret_err!(MsgId::try_from(msg_id), "".into());

        ret_none!(
            &ret_none!(self.map.get(&msg_id), "".to_owned()).body,
            "".to_owned()
        )
        .to_string()
    }

    fn message_author_by_id(&self, msg_id: ffi::MsgIdRef) -> ffi::UserId {
        let msg_id = ret_err!(MsgId::try_from(msg_id), ffi::NULL_USER_ID.to_string());

        ret_none!(self.map.get(&msg_id), ffi::NULL_USER_ID.to_string())
            .author
            .to_string()
    }

    fn op(&self, row_index: usize) -> Option<ffi::MsgIdRef> {
        let mid = self.list.get(row_index)?.msg_id;

        Some(self.map.get(&mid)?.op.as_ref()?.as_slice())
    }

    fn is_reply(&self, row_index: usize) -> bool {
        self.op(row_index).is_some()
    }

    fn delete_message(&mut self, row_index: u64) -> bool {
        let row_index = row_index as usize;

        let id = ret_none!(self.list.get(row_index), false).msg_id;

        match message::delete_message(&id) {
            Ok(_) => {
                self.model.begin_remove_rows(row_index, row_index);
                self.list.remove(row_index);
                self.map.remove(&id);
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

    /// Polls for updates
    fn poll_update(&mut self) -> bool {
        let conv_id = ret_none!(self.conversation_id, false);

        let rx = match MSG_RXS.get(&conv_id) {
            Some(rx) => rx,
            // it's not a problem if the model doesn't have a receiver yet
            None => return true,
        };

        for update in rx.try_iter() {
            match update {
                MsgUpdate::Msg(mid) => {
                    let new = ret_err!(message::get_message(&mid), false);

                    new_msg_toast(&new);

                    self.model
                        .begin_insert_rows(self.list.len(), self.list.len());
                    self.list.push(Message {
                        msg_id: new.message_id,
                        data_saved: true,
                    });
                    self.map.insert(new.message_id, new);
                    self.model.end_insert_rows();

                    self.emit_last_changed();
                }
                MsgUpdate::FullMsg(msg) => {
                    ret_err!(self.raw_insert(msg, false), false);
                }
                MsgUpdate::Receipt(mid) => {
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

                    let receipts = ret_err!(message::get_message_receipts(&mid), false);
                    msg.receipts = receipts;

                    self.model.data_changed(ix, ix);
                }
                MsgUpdate::StoreDone(mid) => {
                    let ix = ret_none!(
                        self.list
                            .iter()
                            // search backwards,
                            // it's probably fairly recent
                            .rposition(|m| m.msg_id == mid),
                        false
                    );
                    self.list[ix].data_saved = true;
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

impl Drop for Messages {
    fn drop(&mut self) {
        let cid = match self.conversation_id {
            Some(cid) => cid,
            None => {
                return;
            }
        };

        MSG_EMITTERS.remove(&cid);
        MSG_RXS.remove(&cid);
        MSG_TXS.remove(&cid);
    }
}
