use crate::{
    err,
    interface::{MessagesEmitter as Emitter, MessagesList as List},
    none,
};
use crossbeam_channel::Sender;
use herald_common::UserId;
use heraldcore::{
    message::{Elider, ReceiptStatus},
    types::*,
};
use messages_helper::{container::Container, search::SearchState};
use search_pattern::SearchPattern;

mod helper_trait_imp;
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

    typing_sender: Option<Sender<()>>,
}

impl Messages {
    pub(crate) fn process_update(
        &mut self,
        update: MsgUpdate,
    ) {
        let emit = &mut self.emit;
        let model = &mut self.model;
        let search = &mut self.search;
        let cid = none!(self.conversation_id);

        let push = |cid| {
            crate::conversation_content::new_activity(cid);
        };

        match update {
            MsgUpdate::NewMsg(new) => {
                self.container
                    .insert_helper(*new, emit, model, search, cid, push);
            }

            MsgUpdate::BuilderMsg(msg) => {
                self.container
                    .insert_helper(*msg, emit, model, search, cid, push);
            }

            MsgUpdate::Receipt {
                msg_id,
                recipient,
                status,
            } => {
                self.container
                    .handle_receipt(msg_id, status, recipient, model, emit, cid);
            }

            MsgUpdate::Reaction {
                msg_id,
                reactionary,
                content,
                remove,
            } => {
                self.container
                    .handle_reaction(msg_id, reactionary, content, remove, model);
            }

            MsgUpdate::StoreDone(mid, meta) => {
                self.container
                    .handle_store_done(mid, meta, emit, model, cid);
            }

            MsgUpdate::SendDone(mid) => {
                self.container.handle_send_done(mid, model, emit, cid);
            }

            MsgUpdate::ExpiredMessages(mids) => {
                self.container
                    .handle_expiration(mids, emit, model, search, &mut self.builder, cid)
            }

            MsgUpdate::Container(container) => {
                if container.is_empty() {
                    return;
                }

                self.model
                    .begin_insert_rows(0, container.len().saturating_sub(1));
                self.container = *container;
                self.model.end_insert_rows();
                self.emit_last_changed();
            }
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    pub(crate) fn last_msg_id(&self) -> Option<MsgId> {
        self.container.list.last().map(|m| m.msg_id)
    }
}

/// Message related conversation updates
#[derive(Debug)]
pub(crate) enum MsgUpdate {
    /// A new message
    NewMsg(Box<heraldcore::message::Message>),

    /// A message has been acknowledged
    Receipt {
        msg_id: MsgId,
        recipient: UserId,
        status: ReceiptStatus,
    },

    /// A reaction has been added or removed
    Reaction {
        msg_id: MsgId,
        reactionary: UserId,
        content: heraldcore::message::ReactContent,
        remove: bool,
    },

    /// A rendered message from the `MessageBuilder`
    BuilderMsg(Box<heraldcore::message::Message>),

    /// An outbound message has been saved
    StoreDone(MsgId, herald_attachments::AttachmentMeta),

    /// There are expired messages that need to be pruned
    ExpiredMessages(Vec<MsgId>),

    /// The container contents, sent when the conversation id is first set.
    Container(Box<Container>),

    /// An outbound message has arrived at the server
    SendDone(MsgId),
}
