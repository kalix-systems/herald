use crate::interface::{ErrorsEmitter as Emitter, ErrorsTrait as Interface};
use std::collections::VecDeque;

/// Errors
pub struct Errors {
    emit: Emitter,
    try_poll: bool,
    errors: VecDeque<String>,
}

impl Interface for Errors {
    fn new(emit: Emitter) -> Self {
        Errors {
            emit,
            try_poll: false,
            errors: VecDeque::new(),
        }
    }

    fn try_poll(&self) -> bool {
        self.try_poll
    }

    fn next_error(&mut self) -> String {
        self.errors.pop_back().unwrap_or_default()
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }
}

impl Errors {
    pub(crate) fn handle_error(
        &mut self,
        error: String,
    ) {
        self.try_poll = !self.try_poll;
        self.errors.push_front(error);
        self.emit.try_poll_changed();
    }
}
