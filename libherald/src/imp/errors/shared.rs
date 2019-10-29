use crate::interface::ErrorsEmitter;
use crate::shared::SingletonBus;
use crossbeam_channel::*;
use heraldcore::{channel_send_err, NE};
use lazy_static::*;
use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc,
};

type Emitter = ErrorsEmitter;

type ErrorString = String;

/// Error queue
pub struct ErrorBus {
    pub(crate) rx: Receiver<ErrorString>,
    pub(crate) tx: Sender<ErrorString>,
}

impl ErrorBus {
    /// Creates new `ConvChannel`
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self { rx, tx }
    }
}

lazy_static! {
    /// Global error queue
    pub static ref ERROR_BUS: ErrorBus = ErrorBus::new();

    /// Errors emitter, filled in when the errors object is constructed
    pub static ref ERROR_EMITTER: Mutex<Option<Emitter>> = Mutex::new(None);

    /// Data associated with `ERROR_EMITTER`.
    pub static ref ERROR_TRY_POLL: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
}

impl SingletonBus for super::Errors {
    type Update = ErrorString;

    fn push(update: Self::Update) -> Result<(), heraldcore::errors::HErr> {
        ERROR_BUS
            .tx
            .clone()
            .send(update)
            .map_err(|_| channel_send_err!())?;
        error_emit_try_poll().ok_or(NE!())?;
        Ok(())
    }
}

/// Emits a signal to the QML runtime, returns `None` on failure.
pub fn error_emit_try_poll() -> Option<()> {
    let mut lock = ERROR_EMITTER.lock();
    let emitter = lock.as_mut()?;

    ERROR_TRY_POLL.fetch_add(1, Ordering::Acquire);
    emitter.try_poll_changed();
    Some(())
}
