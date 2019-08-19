use crate::interface::*;
use heraldcore::config::Config;

pub struct HeraldState {
    emit: HeraldStateEmitter,
}

impl HeraldStateTrait for HeraldState {
    fn new(emit: HeraldStateEmitter) -> Self {
        HeraldState { emit }
    }

    fn emit(&mut self) -> &mut HeraldStateEmitter {
        &mut self.emit
    }

    fn create_min_config(&mut self, id: String) {
        if let Err(e) = Config::add(id.as_str(), None, None) {
            eprintln!("Error: {}", e);
        }
    }
}
