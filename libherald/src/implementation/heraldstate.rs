use crate::interface::*;
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
        if let Err(e) = Config::create_table() {
            eprintln!("{}", e);
            return false;
        }
        if let Err(e) = heraldcore::message::Messages::create_table() {
            eprintln!("{}", e);
        }
        if let Err(e) = heraldcore::contact::ContactsHandle::create_table() {
            eprintln!("{}", e);
        }
        if let Err(e) = heraldcore::members::Members::create_table() {
            eprintln!("{}", e);
        }
        if let Err(e) = heraldcore::conversation::Conversations::create_table() {
            eprintln!("{}", e);
        }

        if let Err(e) = ConfigBuilder::new(id).add() {
            eprintln!("{}", e);
            return false;
        }

        self.emit.config_init_changed();

        true
    }

    fn emit(&mut self) -> &mut HeraldStateEmitter {
        &mut self.emit
    }
}
