use super::*;
use coremacros::from_fn;
use herald_attachments::{Attachment, AttachmentMeta};

#[derive(Default)]
/// Builder for storing outbound messages
pub struct OutboundMessageBuilder {
    /// Recipient user id
    pub conversation: Option<ConversationId>,
    /// Body of message
    pub body: Option<MessageBody>,
    /// Message id of the message being replied to
    pub op: Option<MsgId>,
    /// Attachments
    pub attachments: Vec<PathBuf>,
    /// Expiration period
    pub expiration_period: Option<coretypes::conversation::ExpirationPeriod>,
}

/// Values `OutboundMessageBuilder`'s `store_and_send` function produces
/// while sending the message
#[derive(Debug, Clone)]
pub enum StoreAndSend {
    /// The message being stored and sent
    Msg(ConversationId, Box<Message>),
    /// A signal that the message has been stored successfully
    StoreDone(ConversationId, MsgId, AttachmentMeta),
    /// A signal that the message has been sent
    SendDone(ConversationId, MsgId),
}

// implement conversion
from_fn!(
    crate::updates::Notification,
    StoreAndSend,
    crate::updates::Notification::OutboundMsg
);

impl OutboundMessageBuilder {
    /// Set conversation id
    pub fn conversation_id(
        &mut self,
        conversation_id: ConversationId,
    ) -> &mut Self {
        self.conversation.replace(conversation_id);
        self
    }

    /// Set body
    pub fn body(
        &mut self,
        body: MessageBody,
    ) -> &mut Self {
        self.body.replace(body);
        self
    }

    /// Set the id of the message being replied to, if this message is a reply
    pub fn replying_to(
        &mut self,
        op_msg_id: Option<MsgId>,
    ) -> &mut Self {
        self.op = op_msg_id;
        self
    }

    /// Add attachment
    pub fn add_attachment(
        &mut self,
        path: PathBuf,
    ) -> &mut Self {
        self.attachments.push(path);
        self
    }

    /// Stores and sends the message
    pub fn store_and_send(self) -> Result<(), HErr> {
        let mut db = Database::get()?;
        self.store_and_send_db(&mut db);
        Ok(())
    }

    /// Stores the message without sending it. This function is meant for testing
    /// and not intended to be used outside of this workspace.
    pub fn store(self) -> Result<Message, HErr> {
        let mut db = Database::get()?;
        self.store_db(&mut db)
    }
}

#[derive(Default)]
pub(crate) struct InboundMessageBuilder {
    /// Local message id
    pub(crate) message_id: Option<MsgId>,
    /// Author user id
    pub(crate) author: Option<UserId>,
    /// Recipient user id
    pub(crate) conversation: Option<ConversationId>,
    /// Body of message
    pub(crate) body: Option<MessageBody>,
    /// Time the was received at the server.
    pub(crate) server_timestamp: Option<Time>,
    /// Time the message expires
    pub(crate) expiration: Option<Time>,
    /// Message id of the message being replied to
    pub(crate) op: Option<MsgId>,
    /// Message attachments
    pub(crate) attachments: Vec<Attachment>,
}

impl InboundMessageBuilder {
    pub(crate) fn id(
        &mut self,
        msg_id: MsgId,
    ) -> &mut Self {
        self.message_id.replace(msg_id);
        self
    }

    pub(crate) fn author(
        &mut self,
        uid: UserId,
    ) -> &mut Self {
        self.author.replace(uid);
        self
    }

    pub(crate) fn conversation_id(
        &mut self,
        conversation_id: ConversationId,
    ) -> &mut Self {
        self.conversation.replace(conversation_id);
        self
    }

    #[allow(unused)]
    pub(crate) fn body(
        &mut self,
        body: MessageBody,
    ) -> &mut Self {
        self.body.replace(body);
        self
    }

    pub(crate) fn timestamp(
        &mut self,
        ts: Time,
    ) -> &mut Self {
        self.server_timestamp.replace(ts);
        self
    }

    #[allow(unused)]
    pub(crate) fn replying_to(
        &mut self,
        op_msg_id: MsgId,
    ) -> &mut Self {
        self.op.replace(op_msg_id);
        self
    }

    pub(crate) fn attachments(
        &mut self,
        attachments: Vec<Attachment>,
    ) -> &mut Self {
        self.attachments = attachments;
        self
    }

    #[allow(unused)]
    pub(crate) fn expiration(
        &mut self,
        expiration: Time,
    ) -> &mut Self {
        self.expiration.replace(expiration);
        self
    }

    pub fn store(self) -> Result<Option<Message>, HErr> {
        let mut db = Database::get()?;
        self.store_db(&mut db)
    }
}
