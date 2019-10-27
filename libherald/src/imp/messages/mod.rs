use crate::{ffi, interface::*, ret_err, ret_none, shared::SingletonBus, toasts::new_msg_toast};
use herald_common::{Time, UserId};
use heraldcore::{
    abort_err,
    config::Config,
    conversation,
    errors::HErr,
    message::{self, Message as Msg},
    types::*,
    NE,
};
use im_rc::vector::Vector;
use std::{
    cmp::Ordering,
    collections::HashMap,
    convert::{TryFrom, TryInto},
    ops::Drop,
};

pub(crate) mod shared;
use shared::*;

type Emitter = MessagesEmitter;
type List = MessagesList;

#[derive(Clone, PartialEq, Eq)]
/// A thin wrapper around a `MsgId`
pub struct Message {
    msg_id: MsgId,
    data_saved: bool,
    insertion_time: Time,
}

impl PartialOrd for Message {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.insertion_time.0.partial_cmp(&rhs.insertion_time.0)
    }
}

impl Ord for Message {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match self.partial_cmp(rhs) {
            Some(ord) => {
                return ord;
            }
            None => self.msg_id.cmp(&rhs.msg_id),
        }
    }
}

/// A wrapper around a vector of `Message`s with additional fields
/// to facilitate interaction with QML.
pub struct Messages {
    emit: Emitter,
    model: List,
    list: Vector<Message>,
    map: HashMap<MsgId, Msg>,
    local_id: UserId,
    conversation_id: Option<ConversationId>,
}

impl MessagesTrait for Messages {
    fn new(emit: Emitter, model: List) -> Self {
        Messages {
            list: Vector::new(),
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
        Some(self.last_msg()?.time.insertion.0)
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
        if let (Some(id), None) = (conversation_id, self.conversation_id) {
            let conversation_id = ret_err!(ConversationId::try_from(id));

            EMITTERS.insert(conversation_id, self.emit().clone());

            self.conversation_id = Some(conversation_id);
            self.emit.conversation_id_changed();

            let messages: Vector<Message> =
                ret_err!(conversation::conversation_messages(&conversation_id))
                    .into_iter()
                    .map(|m| {
                        let mid = m.message_id;
                        let insertion_time = m.time.insertion;
                        self.map.insert(mid, m);
                        Message {
                            msg_id: mid,
                            data_saved: true,
                            insertion_time,
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

        match self.map.get(&msg_id) {
            Some(msg) => match msg.body.as_ref() {
                Some(body) => body.to_string(),
                None => "REDACTED".to_owned(),
            },
            None => "REDACTED".to_owned(),
        }
    }

    fn message_author_by_id(&self, msg_id: ffi::MsgIdRef) -> ffi::UserId {
        let msg_id = ret_err!(MsgId::try_from(msg_id), ffi::NULL_USER_ID.to_string());

        match self.map.get(&msg_id) {
            Some(msg) => msg.author.to_string(),
            None => "UNKNOWN".to_owned(),
        }
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

        self.model
            .begin_remove_rows(0, self.list.len().saturating_sub(1));
        self.list = Vector::new();
        self.map = HashMap::new();
        self.model.end_remove_rows();

        self.emit_last_changed();
        true
    }

    fn epoch_timestamp_ms(&self, row_index: usize) -> i64 {
        let mid = ret_none!(self.list.get(row_index), 0).msg_id;

        ret_none!(self.map.get(&mid), 0).time.insertion.0
    }

    fn can_fetch_more(&self) -> bool {
        let conv_id = match &self.conversation_id {
            Some(cid) => cid,
            None => return false,
        };

        let rx = match RXS.get(conv_id) {
            Some(rx) => rx,
            // it's not a problem if the model doesn't have a receiver yet
            None => return false,
        };

        !rx.is_empty()
    }

    /// Polls for updates
    fn fetch_more(&mut self) {
        let conv_id = ret_none!(self.conversation_id);

        let rx = match RXS.get(&conv_id) {
            Some(rx) => rx,
            // it's not a problem if the model doesn't have a receiver yet
            None => return,
        };

        for update in rx.try_iter() {
            match update {
                MsgUpdate::Msg(mid) => {
                    let new = ret_err!(message::get_message(&mid));

                    new_msg_toast(&new);

                    self.model
                        .begin_insert_rows(self.list.len(), self.list.len());
                    self.list.push_back(Message {
                        msg_id: new.message_id,
                        data_saved: true,
                        insertion_time: new.time.insertion,
                    });
                    self.map.insert(new.message_id, new);
                    self.model.end_insert_rows();

                    self.emit_last_changed();
                }
                MsgUpdate::FullMsg(msg) => {
                    ret_err!(self.raw_insert(msg, false));
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
                    let ix = ret_none!(self
                        .list
                        .iter()
                        // search backwards,
                        // it's probably fairly recent
                        .rposition(|m| m.msg_id == mid));

                    let receipts = ret_err!(message::get_message_receipts(&mid));
                    msg.receipts = receipts;

                    self.model.data_changed(ix, ix);
                }
                MsgUpdate::StoreDone(mid) => {
                    let ix = ret_none!(self
                        .list
                        .iter()
                        // search backwards,
                        // it's probably fairly recent
                        .rposition(|m| m.msg_id == mid));
                    self.list[ix].data_saved = true;
                    self.model.data_changed(ix, ix);
                }
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
        let insertion_time = msg.time.insertion;
        let cid = self.conversation_id.ok_or(NE!())?;
        self.model
            .begin_insert_rows(self.row_count(), self.row_count());
        self.list.push_back(Message {
            msg_id,
            data_saved,
            insertion_time,
        });
        self.map.insert(msg_id, msg);
        self.model.end_insert_rows();

        self.emit_last_changed();

        use crate::imp::conversations::{shared::*, Conversations};
        Conversations::push(ConvUpdates::NewActivity(cid))?;

        Ok(())
    }
}

impl Drop for Messages {
    fn drop(&mut self) {
        if let Some(cid) = self.conversation_id {
            EMITTERS.remove(&cid);
            TXS.remove(&cid);
            RXS.remove(&cid);
        }
    }
}
