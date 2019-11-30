use super::*;
use std::collections::HashMap;

/// An event. These are produced in response a message being received from the server.
#[derive(Debug)]
pub struct Event {
    notifications: Vec<Notification>,
    creplies: HashMap<ConversationId, Vec<ConversationMessage>>,
    areplies: HashMap<UserId, Vec<AuxMessage>>,
    errors: Vec<HErr>,
}

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
        self.creplies.entry(to).or_insert(Vec::new()).push(content)
    }

    pub fn add_areply(
        &mut self,
        to: UserId,
        content: AuxMessage,
    ) {
        self.areplies.entry(to).or_insert(Vec::new()).push(content)
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
        let Event {
            mut notifications,
            creplies,
            areplies,
            errors,
        } = other;

        self.notifications.append(&mut notifications);

        for (c, mut r) in creplies.into_iter() {
            if let Some(r0) = self.creplies.get_mut(&c) {
                r0.append(&mut r);
            } else {
                self.creplies.insert(c, r);
            }
        }

        for (u, mut r) in areplies.into_iter() {
            if let Some(r0) = self.areplies.get_mut(&u) {
                r0.append(&mut r);
            } else {
                self.areplies.insert(u, r);
            }
        }
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
            creplies,
            areplies,
        } = self;

        for note in notifications {
            f(note);
        }

        for herr in errors {
            g(herr);
        }

        for (uid, contents) in areplies {
            for content in contents {
                send_amessage(uid, &content)?;
            }
        }

        for (cid, contents) in creplies {
            for content in contents.iter() {
                send_cmessage(cid, content)?;
            }
        }

        Ok(())
    }
}

impl Default for Event {
    fn default() -> Self {
        Event {
            notifications: Vec::new(),
            creplies: HashMap::new(),
            areplies: HashMap::new(),
            errors: Vec::new(),
        }
    }
}
