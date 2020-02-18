use super::*;
use crypto_store::prelude as cstore;
use ratchet_chat::protocol as proto;

/// An event. These are produced in response a message being received from the server.
#[derive(Debug, Default)]
pub struct Event {
    pub(super) notifications: Vec<Notification>,
    pub(super) outbox: Vec<(Recip, proto::Msg)>,
    pub(super) errors: Vec<HErr>,
}

impl Event {
    /// Merges two events.
    pub fn merge(
        &mut self,
        mut other: Self,
    ) {
        self.notifications.append(&mut other.notifications);
        self.outbox.append(&mut other.outbox);
        // self.replies.append(&mut other.replies);
    }

    /// Sends replies to inbound messages and calls `f`, passing each notification in as an
    /// argument.
    pub fn execute(self) -> Result<(), HErr> {
        todo!()
        // let Event {
        //     notifications,
        //     errors,
        //     replies,
        // } = self;

        // for note in notifications {
        //     crate::push(note);
        // }

        // for herr in errors {
        //     crate::err(herr);
        // }

        // for (cid, content) in replies {
        //     w!(send_cmessage(cid, &content));
        // }

        // Ok(())
    }

    pub fn add_msg_to_self(
        &mut self,
        msg: proto::Msg,
    ) -> Result<(), HErr> {
        let uid = w!(config::id());
        self.outbox.push((Recip::One(SingleRecip::User(uid)), msg));
        Ok(())
    }

    pub fn add_msg_to_user(
        &mut self,
        to: UserId,
        msg: proto::Msg,
    ) {
        self.outbox.push((Recip::One(SingleRecip::User(to)), msg));
    }

    pub fn add_msg_to_device(
        &mut self,
        to: sig::PublicKey,
        msg: proto::Msg,
    ) {
        self.outbox.push((Recip::One(SingleRecip::Key(to)), msg));
    }

    pub fn push_cm(
        &mut self,
        cid: ConversationId,
        msg: &ConversationMessage,
    ) -> Result<(), HErr> {
        let uid = w!(config::id());
        let kp = w!(config::keypair());

        let raw = cstore::raw_conn();
        let mut lock = raw.lock();
        let mut store = w!(cstore::as_conn(&mut lock));
        let payload: proto::Payload = kson::to_vec(msg).into();

        let to = w!(crate::members::members(&cid));

        for user in to {
            let pairs = w!(proto::prepare_send_to_user(
                &mut store,
                &kp,
                user,
                payload.clone()
            ));

            self.outbox.extend(
                pairs
                    .into_iter()
                    .map(|(k, m)| (Recip::One(SingleRecip::Key(k)), m)),
            );
        }

        Ok(())
    }
}
