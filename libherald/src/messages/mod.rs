use crate::{
    interface::{MessagesEmitter as Emitter, MessagesList as List},
    ret_err, ret_none,
    toasts::new_msg_toast,
};
use herald_common::UserId;
use heraldcore::{conversation, errors::HErr, message::MessageReceiptStatus, types::*, NE};
use im::vector::Vector;
use messages_helper::search::SearchState;
use search_pattern::SearchPattern;
use std::collections::HashMap;

mod container;
use container::*;
mod imp;
mod trait_imp;
pub(crate) mod underscore;

/// Implementation of `crate::interface::MessageBuilderTrait`.
pub mod builder;
use builder::MessageBuilder;

/// A wrapper around a vector of `Message`s with additional fields
/// to facilitate interaction with QML.
pub struct Messages {
    emit: Emitter,
    model: List,
    local_id: Option<UserId>,
    conversation_id: Option<ConversationId>,
    container: Container,
    search: SearchState,
    builder: MessageBuilder,
    elider: Elider,
}

impl Messages {
    pub(crate) fn process_update(
        &mut self,
        update: MsgUpdate,
    ) {
        match update {
            MsgUpdate::NewMsg(new) => {
                new_msg_toast(new.as_ref());

                ret_err!(self.insert_helper(*new));
            }
            MsgUpdate::BuilderMsg(msg) => {
                ret_err!(self.insert_helper(*msg));
            }
            MsgUpdate::Receipt {
                msg_id,
                recipient,
                status,
            } => {
                ret_err!(container::handle_receipt(
                    &mut self.container,
                    msg_id,
                    status,
                    recipient,
                    &mut self.model
                ));
            }
            MsgUpdate::StoreDone(mid, meta) => {
                ret_none!(container::handle_store_done(
                    &mut self.container,
                    mid,
                    meta,
                    &mut self.model
                ));
            }
            MsgUpdate::ExpiredMessages(mids) => self.handle_expiration(mids),
            MsgUpdate::Container(container) => {
                if container.is_empty() {
                    return;
                }

                self.model
                    .begin_insert_rows(0, container.len().saturating_sub(1));
                self.container = container;
                self.model.end_insert_rows();
                self.emit.is_empty_changed();
                self.emit_last_changed();
            }
        }
    }
}

/// Message related conversation updates
pub(crate) enum MsgUpdate {
    /// A new message
    NewMsg(Box<heraldcore::message::Message>),
    /// A message has been acknowledged
    Receipt {
        msg_id: MsgId,
        recipient: UserId,
        status: MessageReceiptStatus,
    },
    /// A rendered message from the `MessageBuilder`
    BuilderMsg(Box<heraldcore::message::Message>),
    /// Save is complete
    StoreDone(MsgId, heraldcore::message::attachments::AttachmentMeta),
    /// There are expired messages that need to be pruned
    ExpiredMessages(Vec<MsgId>),
    /// The container contents, sent when the conversation id is first set.
    Container(Container),
}
