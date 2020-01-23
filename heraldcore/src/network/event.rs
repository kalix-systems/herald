use super::*;
use crypto_store::prelude::Msg;

/// An event. These are produced in response a message being received from the server.
#[derive(Debug)]
pub struct Event {
    pub(super) notifications: Vec<Notification>,
    pub(super) replies: Vec<(sig::PublicKey, Msg)>,
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

    /// Adds a reply
    pub fn reply(
        &mut self,
        pk: sig::PublicKey,
        msg: Msg,
    ) {
        self.replies.push((pk, msg));
    }

    /// Adds replies in bulk
    pub fn replies(
        &mut self,
        mut rs: Vec<(sig::PublicKey, Msg)>,
    ) {
        self.replies.append(&mut rs);
    }

    /// Adds a notification
    pub fn note<T: Into<Notification>>(
        &mut self,
        notif: T,
    ) {
        self.notifications.push(notif.into());
    }

    /// Sends replies to inbound messages sends notifications
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
            //send_cmessage(cid, &content)?;
        }

        Ok(())
    }
}

impl Default for Event {
    fn default() -> Event {
        Event {
            notifications: Default::default(),
            replies: Vec::new(),
            errors: Default::default(),
        }
    }
}
