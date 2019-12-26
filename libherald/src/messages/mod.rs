use crate::{
    err,
    interface::{MessagesEmitter as Emitter, MessagesList as List},
    none,
    toasts::new_msg_toast,
};
use herald_common::UserId;
use heraldcore::{
    conversation,
    errors::HErr,
    message::{Elider, MessageReceiptStatus},
    types::*,
};
use messages_helper::{container::Container, search::SearchState};
use search_pattern::SearchPattern;

mod helpers;
pub(crate) mod imp;
mod trait_imp;

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

                err!(self.insert_helper(*new));
            }
            MsgUpdate::BuilderMsg(msg) => {
                err!(self.insert_helper(*msg));
            }
            MsgUpdate::Receipt {
                msg_id,
                recipient,
                status,
            } => {
                let model = &mut self.model;

                none!(&self
                    .container
                    .handle_receipt(msg_id, status, recipient, |ix| model.data_changed(ix, ix)));
            }
            MsgUpdate::StoreDone(mid, meta) => {
                let model = &mut self.model;

                none!(&self
                    .container
                    .handle_store_done(mid, meta, |ix| model.data_changed(ix, ix)));
            }
            MsgUpdate::SendDone(mid) => {
                let model = &mut self.model;

                none!(&self
                    .container
                    .handle_send_done(mid, |ix| { model.data_changed(ix, ix) }));
            }
            MsgUpdate::ExpiredMessages(mids) => self.handle_expiration(mids),
            MsgUpdate::Container(container) => {
                if container.is_empty() {
                    return;
                }

                self.model
                    .begin_insert_rows(0, container.len().saturating_sub(1));
                self.container = *container;
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
    /// An outbound message has been saved
    StoreDone(MsgId, heraldcore::message::attachments::AttachmentMeta),
    /// There are expired messages that need to be pruned
    ExpiredMessages(Vec<MsgId>),
    /// The container contents, sent when the conversation id is first set.
    Container(Box<Container>),
    /// An outbound message has arrived at the server
    SendDone(MsgId),
}
