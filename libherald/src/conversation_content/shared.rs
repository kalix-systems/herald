use super::*;
use crate::messages::MsgUpdate;
use anyhow::anyhow;
use crossbeam_channel::*;
use heraldcore::conversation::{settings::SettingsUpdate as CoreSettingsUpdate, ExpirationPeriod};
use heraldcore::errors::HErr;
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
    CHANS.get_or_init(Default::default)
}

fn emitters() -> &'static Mutex<HashMap<ConversationId, Emitter>> {
    EMITTERS.get_or_init(Default::default)
}

/// An update to a conversation data
#[derive(Debug)]
pub(crate) enum ContentUpdate {
    /// Messages model update
    Msg(MsgUpdate),
    /// Members model update
    Member(MemberUpdate),
    /// Conversation meta data update
    Meta(MetaUpdate),
}

#[derive(Debug)]
pub enum MetaUpdate {
    /// Expiration period has been changed
    ExpirationChanged(ExpirationPeriod),
    /// Conversation picture has been changed
    PictureChanged(Option<String>),
    /// Conversation title has been changed
    TitleChanged(Option<String>),
    /// Pairwise user data changed
    UserChanged(herald_user::UserChange),
    NewActivity,
}

impl From<CoreSettingsUpdate> for ContentUpdate {
    fn from(update: CoreSettingsUpdate) -> Self {
        use MetaUpdate::*;
        let update = match update {
            CoreSettingsUpdate::Expiration(period) => ExpirationChanged(period),
            CoreSettingsUpdate::Title(title) => TitleChanged(title),
            CoreSettingsUpdate::Picture(path) => PictureChanged(path),
        };

        update.into()
    }
}

impl From<herald_user::UserChange> for ContentUpdate {
    fn from(u: herald_user::UserChange) -> Self {
        ContentUpdate::Meta(MetaUpdate::UserChanged(u))
    }
}

impl From<MemberUpdate> for ContentUpdate {
    fn from(update: MemberUpdate) -> ContentUpdate {
        ContentUpdate::Member(update)
    }
}

impl From<MetaUpdate> for ContentUpdate {
    fn from(update: MetaUpdate) -> ContentUpdate {
        ContentUpdate::Meta(update)
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
                        .map_err(|_| anyhow!("Failed to send update")));

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
                err!(tx
                    .send(update)
                    .map_err(|_| anyhow!("Failed to send update")));

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

pub(crate) fn new_activity(cid: ConversationId) {
    err!(content_push(cid, MetaUpdate::NewActivity));
    crate::push(crate::conversations::shared::ConvUpdate::NewActivity(cid));
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
                Meta(update) => {
                    self.handle_meta_update(update);
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

    fn handle_meta_update(
        &mut self,
        update: MetaUpdate,
    ) -> Option<()> {
        use MetaUpdate::*;

        let mut write = cs::conv_data().write();
        let data = write.get_mut(&self.id?)?;

        match update {
            PictureChanged(path) => {
                data.picture = path;
                drop(write);

                self.emit.picture_changed();
            }
            TitleChanged(title) => {
                data.title = title;
                drop(write);

                self.emit.title_changed();
            }
            ExpirationChanged(period) => {
                data.expiration_period = period;
                drop(write);

                self.emit.expiration_period_changed();
            }
            UserChanged(update) => {
                use herald_user::UserChange as U;

                if data.pairwise_uid.is_none() {
                    return Some(());
                }

                drop(write);

                match update {
                    U::Picture(_) => {
                        self.emit.picture_changed();
                    }
                    U::Color(_) => {
                        self.emit.conversation_color_changed();
                    }
                    U::DisplayName(_) => {
                        self.emit.title_changed();
                    }
                }
            }
            NewActivity => {
                use heraldcore::conversation::Status;
                if data.status == Status::Archived {
                    data.status = Status::Active;
                    self.emit.status_changed();
                }
            }
        };

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
