use super::*;
use crate::updates::push;
use std::{thread, time::Duration};
pub(crate) mod db;

#[cfg(test)]
mod tests;

/// Polling interval for the GC thread in milliseconds.
///
/// Currently set to five seconds.
const POLL_INTERVAL: u64 = 5_000;

/// Messages to be removed
pub type ConvMessages = HashMap<ConversationId, Vec<MsgId>>;

/// Returns a vector of conversation id's with expired messages.
pub fn get_stale_conversations() -> Result<ConvMessages, HErr> {
    let db = Database::get()?;
    db::get_stale_conversations(&db)
}

/// Deletes expired messages from the database
pub fn delete_expired() -> Result<(), HErr> {
    let db = Database::get()?;
    db::delete_expired(&db)
}

impl From<ConvMessages> for crate::updates::Notification {
    fn from(v: ConvMessages) -> Self {
        crate::updates::Notification::GC(v)
    }
}

/// Initializes the garbage collection thread. Expired messages are sent to the notification
/// stream.
///
/// This function should not be called until the rest of the application state is
/// properly initialized.
///
/// Returns an error if the thread cannot be spawned.
pub fn init() -> Result<(), HErr> {
    thread::Builder::new().spawn(move || {
        let poll_interval = Duration::from_millis(POLL_INTERVAL);

        loop {
            match get_stale_conversations() {
                Ok(cids) => {
                    // only send update if not empty
                    if !cids.is_empty() {
                        push(cids);
                        if let Ok(db) = Database::get() {
                            drop(super::attachments::db::gc(&db));
                        }
                    }
                }
                Err(e) => {
                    crate::err(e);
                }
            }

            if let Err(e) = delete_expired() {
                crate::err(e);
            }

            thread::sleep(poll_interval);
        }
    })?;

    Ok(())
}
