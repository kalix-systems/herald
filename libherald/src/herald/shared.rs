use super::Emitter;
use crossbeam_channel::*;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum RegistrationFailureCode {
    UserIdTaken = 0,
    KeyTaken = 1,
    BadSignature = 2,
    Other = 3,
}

impl Into<Update> for RegistrationFailureCode {
    fn into(self) -> Update {
        Update::RegistrationFailed(self)
    }
}

pub enum Update {
    RegistrationSuccess,
    RegistrationFailed(RegistrationFailureCode),
    Conv(crate::conversations::shared::ConvUpdate),
    User(crate::users::shared::UserUpdate),
    Conf(crate::config::ConfUpdate),
    Error(String),
    // This is here because rust doesn't have specialization
    Nil,
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

static BUS: OnceCell<Bus> = OnceCell::new();

/// `Herald` emitter, filled in when `Herald` is constructed
static EMITTER: OnceCell<Mutex<Emitter>> = OnceCell::new();

fn emitter() -> Option<&'static Mutex<Emitter>> {
    EMITTER.get()
}

fn bus() -> &'static Bus {
    BUS.get_or_init(Bus::new)
}

/// Emits a signal to the QML runtime, returns `None` if the emitter has not been provided yet.
fn emit_new_data() -> Option<()> {
    let mut emitter = emitter()?.lock();

    emitter.try_poll();
    Some(())
}

pub(crate) fn push<T: Into<Update>>(update: T) {
    let update = update.into();
    drop(std::thread::Builder::new().spawn(move || {
        // if this fails, our typical error reporting machinery is broken
        // (which would be odd given that the channel should never be dropped)
        // but we might want to log it some other way. TODO?
        if bus().tx.clone().send(update).is_ok() {
            emit_new_data();
        }
    }));
}

pub(super) fn set_emitter(emit: crate::interface::HeraldEmitter) {
    drop(EMITTER.set(Mutex::new(emit)));
}

pub(super) fn updates() -> TryIter<'static, Update> {
    bus().rx.try_iter()
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
                RegistrationFailed(code) => {
                    self.registration_failure_code.replace(code);
                    self.emit.registration_failure_code_changed();
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
                Nil => {}
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

impl From<()> for Update {
    fn from(_: ()) -> Update {
        Update::Nil
    }
}
