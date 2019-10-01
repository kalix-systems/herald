use crate::interface::*;
use heraldcore::{abort_err, config::Config};

pub struct HeraldState {
    config_init: bool,
    emit: HeraldStateEmitter,
}

impl HeraldStateTrait for HeraldState {
    fn new(emit: HeraldStateEmitter) -> Self {
        let config_init = if Config::static_id().is_ok() {
            true
        } else {
            abort_err!(heraldcore::db::init());
            false
        };

        HeraldState { emit, config_init }
    }

    fn config_init(&self) -> bool {
        self.config_init
    }

    fn set_config_init(&mut self, val: bool) {
        self.config_init |= val;
        self.emit.config_init_changed();
    }

    fn emit(&mut self) -> &mut HeraldStateEmitter {
        &mut self.emit
    }
}
