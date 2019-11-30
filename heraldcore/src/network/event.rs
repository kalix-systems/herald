use super::*;
use std::collections::HashMap;

/// An event. These are produced in response a message being received from the server.
#[derive(Debug)]
pub struct Event {
    notifications: Vec<Notification>,
    replies: Vec<Dispatch>,
    errors: Vec<HErr>,
}

#[derive(Debug)]
enum Dispatch {
    ADisp(UserId, AuxMessage),
    CDisp(ConversationId, ConversationMessage),
    DDisp(sig::PublicKey, DeviceMessageBody),
}

use Dispatch::*;

impl Event {
    pub fn add_notif(
        &mut self,
        notif: Notification,
    ) {
        self.notifications.push(notif);
    }

    pub fn add_creply(
        &mut self,
        to: ConversationId,
        content: ConversationMessage,
    ) {
        self.replies.push(CDisp(to, content));
    }

    pub fn add_areply(
        &mut self,
        to: UserId,
        content: AuxMessage,
    ) {
        self.replies.push(ADisp(to, content));
    }

    pub fn add_dreply(
        &mut self,
        to: sig::PublicKey,
        content: DeviceMessageBody,
    ) {
        self.replies.push(DDisp(to, content));
    }

    pub fn add_error(
        &mut self,
        err: HErr,
    ) {
        self.errors.push(err)
    }

    /// Merges two events.
    pub fn merge(
        &mut self,
        mut other: Self,
    ) {
        self.notifications.append(&mut other.notifications);
        self.replies.append(&mut other.replies);
        self.errors.append(&mut other.errors);
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
            replies,
            mut errors,
        } = self;

        for note in notifications {
            f(note);
        }

        for dispatch in replies {
            match dispatch {
                ADisp(to, content) => {
                    send_amessage(to, &content).unwrap_or_else(|e| errors.push(e))
                }
                CDisp(to, content) => {
                    send_cmessage(to, &content).unwrap_or_else(|e| errors.push(e))
                }
                DDisp(to, content) => {
                    send_dmessage(to, &content).unwrap_or_else(|e| errors.push(e))
                }
            }
        }

        for herr in errors {
            g(herr);
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
