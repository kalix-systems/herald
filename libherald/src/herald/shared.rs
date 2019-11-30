use super::Emitter;
use crossbeam_channel::*;
use heraldcore::{channel_send_err, NE};
use lazy_static::*;
use parking_lot::Mutex;

pub enum Update {
    RegistrationSuccess,
    Conv(crate::conversations::shared::ConvUpdate),
    User(crate::users::shared::UserUpdate),
    Conf(crate::config::ConfUpdate),
    Error(String),
}

/// Channel for updates from `ConversationContent` objects.
struct Bus {
    pub(super) rx: Receiver<Update>,
    pub(super) tx: Sender<Update>,
}

impl Bus {
    /// Creates new `ConvChannel`
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self { rx, tx }
    }
}

lazy_static! {
    static ref BUS: Bus = Bus::new();

    /// `Herald` emitter, filled in when `Herald` is constructed
    static ref EMITTER: Mutex<Option<Emitter>> = Mutex::new(None);
}

/// Emits a signal to the QML runtime, returns `None` if the emitter has not been provided yet.
#[must_use]
fn emit_new_data() -> Option<()> {
    let mut lock = EMITTER.lock();
    let emitter = lock.as_mut()?;

    emitter.new_data_ready();
    Some(())
}

pub(crate) fn push<T: Into<Update>>(update: T) -> Result<(), heraldcore::errors::HErr> {
    BUS.tx
        .clone()
        .send(update.into())
        .map_err(|_| channel_send_err!())?;
    emit_new_data().ok_or(NE!())?;
    Ok(())
}

pub(super) fn set_emitter(emit: crate::interface::HeraldEmitter) {
    let mut lock = EMITTER.lock();
    lock.replace(emit);
}

pub(super) fn updates() -> TryIter<'static, Update> {
    BUS.rx.try_iter()
}

pub(super) fn more_updates() -> bool {
    !BUS.rx.is_empty()
}

impl super::Herald {
    pub(super) fn process_updates(&mut self) {
        for update in updates() {
            use Update::*;
            match update {
                RegistrationSuccess => {
                    self.load_props.setup();
                    self.emit.config_init_changed();
                }
                Conv(update) => {
                    self.load_props.conversations.handle_update(update);
                }
                User(update) => {
                    self.load_props.users.handle_update(update);
                }
                Error(error) => {
                    self.errors.handle_error(error);
                }
                Conf(update) => {
                    self.load_props.config.handle_update(update);
                }
            }
        }
    }
}

impl<U, E> From<(Result<U, E>, location::Location)> for Update
where
    U: Into<Update>,
    E: std::error::Error,
{
    fn from((res, loc): (Result<U, E>, location::Location)) -> Update {
        match res {
            Ok(update) => update.into(),
            Err(e) => Update::Error(format!("{error} at {location}", error = e, location = loc)),
        }
    }
}
