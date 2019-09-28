use crate::{interface::*, ret_err, types::*};
use heraldcore::config::{Config, ConfigBuilder};
use std::convert::TryInto;

pub struct HeraldState {
    config_init: bool,
    emit: HeraldStateEmitter,
}

impl HeraldStateTrait for HeraldState {
    fn new(emit: HeraldStateEmitter) -> Self {
        HeraldState {
            emit,
            config_init: Config::static_id().is_ok(),
        }
    }

    fn config_init(&self) -> bool {
        self.config_init
    }

    fn set_config_id(&mut self, id: FfiUserId) -> bool {
        use heraldcore::*;
        let id = ret_err!(id.as_str().try_into(), false);
        ret_err!(db::init(), false);
        ret_err!(ConfigBuilder::new().id(id).add(), false);

        self.emit.config_init_changed();

        true
    }

    fn emit(&mut self) -> &mut HeraldStateEmitter {
        &mut self.emit
    }
}
