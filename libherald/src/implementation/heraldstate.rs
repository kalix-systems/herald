use crate::{interface::*, ret_err, types::*};
use heraldcore::{
    config::{Config, ConfigBuilder},
    db::DBTable,
};
use std::convert::TryInto;

pub struct HeraldState {
    config_init: bool,
    emit: HeraldStateEmitter,
}

impl HeraldStateTrait for HeraldState {
    fn new(emit: HeraldStateEmitter) -> Self {
        HeraldState {
            emit,
            config_init: Config::exists().unwrap_or(false),
        }
    }

    fn config_init(&self) -> bool {
        self.config_init
    }

    fn set_config_id(&mut self, id: FfiUserId) -> bool {
        use heraldcore::*;
        let id = ret_err!(id.as_str().try_into(), false);
        ret_err!(Config::create_table(), false);
        ret_err!(message::Messages::create_table(), false);
        ret_err!(contact::ContactsHandle::create_table(), false);
        ret_err!(members::Members::create_table(), false);
        ret_err!(conversation::Conversations::create_table(), false);
        ret_err!(ConfigBuilder::new().id(id).add(), false);

        self.emit.config_init_changed();

        true
    }

    fn emit(&mut self) -> &mut HeraldStateEmitter {
        &mut self.emit
    }
}
