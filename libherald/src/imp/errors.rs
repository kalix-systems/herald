use crate::{
    interface::*,
    shared::errors::{ERROR_EMITTER, ERROR_QUEUE, ERROR_TRY_POLL},
};
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc,
};

type Emitter = ErrorsEmitter;

/// Errors
pub struct Errors {
    emit: Emitter,
    try_poll: Arc<AtomicU8>,
}

impl ErrorsTrait for Errors {
    fn new(mut emit: Emitter) -> Self {
        let global_emit = emit.clone();

        ERROR_EMITTER.lock().replace(global_emit);
        let try_poll = ERROR_TRY_POLL.clone();
        Errors { emit, try_poll }
    }

    fn try_poll(&self) -> u8 {
        self.try_poll.load(Ordering::Acquire)
    }

    fn next_error(&mut self) -> String {
        match ERROR_QUEUE.rx.try_recv() {
            Ok(e) => e,
            Err(_) => {
                eprintln!("Couldn't receive error");
                String::new()
            }
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }
}
