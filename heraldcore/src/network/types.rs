use super::*;

#[derive(Copy, Clone, Debug)]
/// `Notification`s contain info about what updates were made to the database.
pub enum Notification {
    /// A new message has been received.
    NewMsg(MsgId, ConversationId),
    /// A message has been received.
    MsgReceipt {
        /// The message that was received
        mid: MsgId,
        /// The conversation the message was part of
        cid: ConversationId,
    },
    /// A new contact has been added
    NewContact(UserId, ConversationId),
    /// A new conversation has been added
    NewConversation(ConversationId),
    /// Response to contact request.
    AddContactResponse(ConversationId, UserId, bool),
    /// Response to request to join conversation.
    AddConversationResponse(ConversationId, UserId, bool),
    /// The conversation settings have been updated
    Settings(ConversationId, settings::SettingsUpdate),
}

/// An event. These are produced in response a message being received from the server.
#[derive(Debug)]
pub struct Event {
    pub(super) notifications: Vec<Notification>,
    pub(super) replies: Vec<(ConversationId, ConversationMessageBody)>,
    pub(super) errors: Vec<HErr>,
}

impl Event {
    /// Merges two events.
    pub fn merge(&mut self, mut other: Self) {
        self.notifications.append(&mut other.notifications);
        self.replies.append(&mut other.replies);
    }

    /// Sends replies to inbound messages and calls `f`, passing each notification in as an
    /// argument.
    pub fn execute<F: FnMut(Notification), G: FnMut(HErr)>(
        self,
        f: &mut F,
        g: &mut G,
    ) -> Result<(), HErr> {
        let Event {
            notifications,
            errors,
            replies,
        } = self;

        for note in notifications {
            f(note);
        }

        for herr in errors {
            g(herr);
        }

        for (cid, content) in replies {
            send_cmessage(cid, &content)?;
        }

        Ok(())
    }
}

impl Default for Event {
    fn default() -> Self {
        Event {
            notifications: Vec::new(),
            replies: Vec::new(),
            errors: Vec::new(),
        }
    }
}
