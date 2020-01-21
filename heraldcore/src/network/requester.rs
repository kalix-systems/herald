use super::*;
use anyhow::anyhow;
use herald_network::Requester;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;

static HANDLE: OnceCell<Mutex<Requester>> = OnceCell::new();

pub(super) fn update(handle: Requester) {
    // try to set the client
    if let Err(handle) = HANDLE.set(Mutex::new(handle)) {
        // if it fails, this means the handle is already set, so we should replace it
        if let Some(slot) = HANDLE.get() {
            let mut lock = slot.lock();
            *lock = handle.into_inner();
        }
    }
}

pub(super) fn send<R: Into<Request>, F: FnMut(Result<Response, HErr>) + Send + 'static>(
    req: R,
    f: F,
) -> Result<(), HErr> {
    let handle: &Mutex<Requester> = HANDLE
        .get()
        .ok_or_else(|| anyhow!("Request handle not set"))?;

    handle.lock().send(req, f)?;

    Ok(())
}
