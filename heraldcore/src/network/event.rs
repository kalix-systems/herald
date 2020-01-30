use super::*;

/// An event. These are produced in response a message being received from the server.
#[derive(Debug)]
pub struct Event {
    pub(super) notifications: Vec<Notification>,
    pub(super) replies: Vec<(ConversationId, ConversationMessage)>,
    pub(super) errors: Vec<HErr>,
}

impl Event {
    /// Merges two events.
    pub fn merge(
        &mut self,
        mut other: Self,
    ) {
        self.notifications.append(&mut other.notifications);
        self.replies.append(&mut other.replies);
    }

    /// Sends replies to inbound messages and calls `f`, passing each notification in as an
    /// argument.
    pub fn execute(self) -> Result<(), HErr> {
        let Event {
            notifications,
            errors,
            replies,
        } = self;

        for note in notifications {
            crate::push(note);
        }

        for herr in errors {
            crate::err(herr);
        }

        for (cid, content) in replies {
            w!(send_cmessage(cid, &content));
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
