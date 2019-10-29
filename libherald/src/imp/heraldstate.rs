use crate::interface::*;
use heraldcore::{abort_err, config::Config, db, message::gc};

/// Global state for the application that can't easily be included
/// in another model. Currently only used to distinguish initial registration
/// from logins.
pub struct HeraldState {
    config_init: bool,
    emit: HeraldStateEmitter,
}

impl HeraldStateTrait for HeraldState {
    fn new(emit: HeraldStateEmitter) -> Self {
        let config_init = if Config::static_id().is_ok() {
            true
        } else {
            // If this fails, the file system is in a very bad place.
            // This probably cannot be recovered from, and there's not meaningful
            // sense in which the application can work.
            //
            // TODO Still, crashing is a bad look and we should at least report this
            // to the UI.
            abort_err!(db::init(), "Couldn't setup database");

            // If this fails, it's because a thread couldn't be spawned.
            // This implies the OS is in a very bad place.
            // We should possibly report this error and continue anyway.
            drop(gc::init(move |update| {
                gc_handler(update);
            }));
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

fn gc_handler(update: gc::GCUpdate) {
    use crate::imp::messages::{shared::MsgUpdate, Messages};
    use crate::shared::AddressedBus;
    use gc::GCUpdate::*;
    match update {
        StaleConversations(cids) => {
            for cid in cids {
                // TODO: push error to error queue
                drop(Messages::push(cid, MsgUpdate::ExpiredMessages));
            }
        }
        GCError(e) => {
            // TODO push to error queue
            eprintln!("{}", e);
        }
    }
}
