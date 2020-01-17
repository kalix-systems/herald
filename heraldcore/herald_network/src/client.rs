use crate::Client;
use once_cell::sync::OnceCell;
use parking_lot::RwLock;

static CLIENT: OnceCell<RwLock<Client>> = OnceCell::new();

pub(crate) fn update(client: Client) {
    // try to set the client
    if let Err(client) = CLIENT.set(RwLock::new(client)) {
        // if it fails, this means the client is already set, so we should replace it
        if let Some(slot) = CLIENT.get() {
            let mut lock = slot.write();
            *lock = client.into_inner();

            drop(lock);
        }
    }
}

pub(crate) fn get() -> Option<&'static RwLock<Client>> {
    CLIENT.get()
}
