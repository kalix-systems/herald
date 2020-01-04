use crate::interface::{ErrorsEmitter as Emitter, ErrorsTrait as Interface};
use std::collections::VecDeque;

/// Errors
pub struct Errors {
    emit: Emitter,
    errors: VecDeque<String>,
}

impl Interface for Errors {
    fn new(emit: Emitter) -> Self {
        Errors {
            emit,
            errors: VecDeque::new(),
        }
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
        self.errors.push_front(error);
        self.emit.new_error();
    }
}
