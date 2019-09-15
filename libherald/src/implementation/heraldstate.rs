use crate::{interface::*, ret_err};
use herald_common::UserId;
use heraldcore::{
    config::{Config, ConfigBuilder},
    db::DBTable,
};

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

    fn set_config_id(&mut self, id: UserId) -> bool {
        use heraldcore::*;
        ret_err!(Config::create_table(), false);
        ret_err!(message::Messages::create_table(), false);
        ret_err!(contact::ContactsHandle::create_table(), false);
        ret_err!(members::Members::create_table(), false);
        ret_err!(conversation::Conversations::create_table(), false);
        ret_err!(ConfigBuilder::new(id).add(), false);

        self.emit.config_init_changed();

        true
    }

    fn emit(&mut self) -> &mut HeraldStateEmitter {
        &mut self.emit
    }
}
