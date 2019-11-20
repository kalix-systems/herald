use super::*;

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
    /// Whether to treat the value as markdown
    pub parse_markdown: bool,
}

/// Values `OutboundMessageBuilder`'s `store_and_send` function
/// can pass into the callback.
#[derive(Debug)]
pub enum StoreAndSend {
    /// The message being stored and sent
    Msg(Box<Message>),
    /// An error accompanied by the line number it occured on
    Error {
        /// The error
        error: HErr,
        /// The line number the error occured on
        line_number: u32,
    },
    /// A signal that the message has been stored successfully
    StoreDone(MsgId),
    /// A signal that the message has been sent
    SendDone(MsgId),
}

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

    /// Parses the text as markdown, if possible, rendering it to HTML
    pub fn parse_markdown(&mut self) -> &mut Self {
        if let Some(body) = &self.body {
            if let Ok(md) = body.parse_markdown() {
                self.body.replace(md);
            }
        }
        self
    }

    /// Stores and sends the message
    pub fn store_and_send<F: FnMut(StoreAndSend) + Send + 'static>(
        self,
        callback: F,
    ) -> Result<(), HErr> {
        let mut db = Database::get()?;
        self.store_and_send_db(&mut db, callback);
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn store_and_send_blocking(self) -> Result<Message, HErr> {
        let mut db = Database::get()?;
        self.store_and_send_blocking_db(&mut db)
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
    pub(crate) attachments: Vec<attachments::Attachment>,
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
        attachments: Vec<attachments::Attachment>,
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
