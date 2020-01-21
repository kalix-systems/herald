use super::*;
use crate::messages::MsgUpdate;
use crossbeam_channel::*;
use heraldcore::{channel_send_err, errors::HErr};
use once_cell::sync::OnceCell;
use parking_lot::{Mutex, RwLock};
use std::collections::HashMap;

struct Bus {
    tx: Sender<ContentUpdate>,
    rx: Receiver<ContentUpdate>,
}

/// Concurrent hash map from `ConversationId`s to an event stream.
/// This is used to route conversation members related notifications that arrive from the network.
static CHANS: OnceCell<RwLock<HashMap<ConversationId, Bus>>> = OnceCell::new();

/// Concurrent hash map of `MembersEmitter`. These are removed when the associated
/// `Members` object is dropped.
static EMITTERS: OnceCell<Mutex<HashMap<ConversationId, Emitter>>> = OnceCell::new();

fn chans() -> &'static RwLock<HashMap<ConversationId, Bus>> {
    CHANS.get_or_init(|| RwLock::new(HashMap::default()))
}

fn emitters() -> &'static Mutex<HashMap<ConversationId, Emitter>> {
    EMITTERS.get_or_init(|| Mutex::new(HashMap::default()))
}

/// An update to a conversation's message model or members model
#[derive(Debug)]
pub(crate) enum ContentUpdate {
    /// Messages model update
    Msg(MsgUpdate),
    /// Members model update
    Member(MemberUpdate),
}

impl From<MemberUpdate> for ContentUpdate {
    fn from(update: MemberUpdate) -> ContentUpdate {
        ContentUpdate::Member(update)
    }
}

impl From<MsgUpdate> for ContentUpdate {
    fn from(update: MsgUpdate) -> ContentUpdate {
        ContentUpdate::Msg(update)
    }
}

pub(crate) fn content_push<T: Into<ContentUpdate>>(
    cid_addr: ConversationId,
    update: T,
) -> Result<(), HErr> {
    let update = update.into();
    std::thread::Builder::new().spawn(move || {
        let chan_lock = chans().read();
        let mut emit_lock = emitters().lock();

        let maybe_tx = chan_lock.get(&cid_addr).map(|b| b.tx.clone());

        match maybe_tx {
            Some(tx) => {
                if let Err(update) = tx.send(update) {
                    drop(chan_lock);
                    let mut chan_lock = chans().write();
                    let (tx, rx) = unbounded();
                    chan_lock.insert(
                        cid_addr,
                        Bus {
                            tx: (&tx).clone(),
                            rx,
                        },
                    );

                    err!(tx
                        .send(update.into_inner())
                        .map_err(|_| channel_send_err!()));

                    drop(chan_lock);
                }
            }
            None => {
                drop(chan_lock);
                let mut chan_lock = chans().write();
                let (tx, rx) = unbounded();
                chan_lock.insert(
                    cid_addr,
                    Bus {
                        tx: (&tx).clone(),
                        rx,
                    },
                );
                err!(tx.send(update).map_err(|_| channel_send_err!()));

                drop(chan_lock);
            }
        };

        let mut maybe_emitter = emit_lock.get_mut(&cid_addr).map(|e| e.clone());

        if let Some(emitter) = maybe_emitter.as_mut() {
            emitter.try_poll();
        }
    })?;
    Ok(())
}

impl super::ConversationContent {
    pub(super) fn process_updates(&mut self) -> Option<()> {
        let id = self.id?;

        for update in chans().read().get(&id)?.rx.try_iter() {
            use ContentUpdate::*;
            match update {
                Msg(update) => {
                    self.messages.process_update(update);
                }
                Member(update) => {
                    self.members.process_update(update);
                }
            }
        }

        Some(())
    }

    #[must_use]
    pub(super) fn register_model(&mut self) -> Option<()> {
        let id = self.id?;

        emitters().lock().insert(id, self.emit.clone());

        Some(())
    }
}

impl Drop for ConversationContent {
    fn drop(&mut self) {
        use shared::*;
        if let Some(cid) = self.id {
            let mut chan_lock = chans().write();
            let mut emitter_lock = emitters().lock();

            emitter_lock.remove(&cid);
            chan_lock.remove(&cid);
        }
    }
}
