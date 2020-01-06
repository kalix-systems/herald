use crate::conversation::settings;
use crate::errors::HErr;
use crate::message;
use coretypes::conversation::ConversationMeta;
use crossbeam_channel::{unbounded, Receiver, Sender};
use herald_common::UserId;
use herald_ids::{ConversationId, MsgId};
use once_cell::sync::OnceCell;

#[derive(Clone, Debug)]
/// `Notification`s contain info about what updates were made to the client's database.
pub enum Notification {
    /// A new message has been received.
    NewMsg(Box<message::Message>),
    /// A message has been received.
    MsgReceipt(message::MessageReceipt),
    /// A message reaction has been received
    Reaction {
        /// Conversation id
        cid: ConversationId,
        /// Message being reacted to
        msg_id: MsgId,
        /// The user that reacted
        reactionary: UserId,
        /// The content of the react
        content: message::ReactContent,
        /// Is this reaction update an addition or a removal?
        remove: bool,
    },
    /// A typing indicator has been received
    TypingIndicator(ConversationId, UserId),
    /// A new user has been added
    NewUser(Box<(herald_user::User, ConversationMeta)>),
    /// A new conversation has been added
    NewConversation(ConversationMeta),
    /// Response to user request.
    AddUserResponse(ConversationId, UserId, bool),
    /// Response to request to join conversation.
    AddConversationResponse(ConversationId, UserId, bool),
    /// The conversation settings have been updated
    Settings(ConversationId, settings::SettingsUpdate),
    /// Message GC thread updates
    GC(crate::message::gc::ConvMessages),
    /// Incremental updates to oubound message sending
    OutboundMsg(crate::message::StoreAndSend),
    /// Incremental updates to outbound auxilliary message sending
    OutboundAux(crate::message::OutboundAux),
    /// User profile information changed
    UserChanged(UserId, herald_user::UserChange),
}

/// Registers handlers for notifications
///
/// # Arguments
/// `mut f: F` - Notication handler
/// `mut g: G` - Error handler
pub fn register_handlers<F, G>(
    mut f: F,
    mut g: G,
) -> Result<(), crate::errors::HErr>
where
    F: FnMut(Notification) + Send + 'static,
    G: FnMut(crate::errors::HErr) + Send + 'static,
{
    std::thread::Builder::new().spawn(move || loop {
        if let Ok(val) = bus().rx.recv() {
            match val {
                Ok(v) => f(v),
                Err(e) => g(e),
            }
        }
    })?;

    Ok(())
}

struct Bus {
    rx: Receiver<Result<Notification, HErr>>,
    tx: Sender<Result<Notification, HErr>>,
}

fn bus() -> &'static Bus {
    BUS.get_or_init(Bus::default)
}

pub(crate) fn push<T: Into<Notification>>(n: T) {
    drop(bus().tx.send(Ok(n.into())));
}

pub(crate) fn err<E: Into<HErr>>(e: E) {
    drop(bus().tx.send(Err(e.into())));
}

impl Default for Bus {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Bus { tx, rx }
    }
}

static BUS: OnceCell<Bus> = OnceCell::new();
