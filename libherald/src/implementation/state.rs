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

    /// TODO implement other variations
    fn create_min_config(&mut self, id: String) {
        if let Err(e) = Config::new(id, None, None) {
            eprintln!("{}", e);
        }
    }
}
