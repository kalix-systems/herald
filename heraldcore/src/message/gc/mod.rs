use super::*;
use std::{thread, time::Duration};
pub(crate) mod db;

#[cfg(test)]
mod tests;

/// Polling interval for the GC thread in milliseconds.
///
/// Currently set to 10 seconds.
const POLL_INTERVAL: u64 = 10 * 60 * 1000;

type ConvMessages = HashMap<ConversationId, Vec<MsgId>>;

/// Values passed into the callback provided to `init`
pub enum GCUpdate {
    /// A vector of conversations with unexpired messages
    StaleConversations(ConvMessages),
    /// An error has occured.
    GCError(HErr),
}

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

/// Initializes the garbage collection thread, taking a callback that is called
/// when stale conversations are found.
///
/// This function should not be called until the rest of the application state is
/// properly initialized.
///
/// Returns an error if the thread cannot be spawned.
pub fn init<F: FnMut(GCUpdate) + Send + 'static>(mut f: F) -> Result<(), HErr> {
    thread::Builder::new().spawn(move || {
        use GCUpdate::*;
        let poll_interval = Duration::from_millis(POLL_INTERVAL);

        loop {
            match get_stale_conversations() {
                Ok(cids) => {
                    // only send update if not empty
                    if !cids.is_empty() {
                        f(StaleConversations(cids));
                    }
                }
                Err(e) => {
                    f(GCError(e));
                }
            }

            if let Err(e) = delete_expired() {
                f(GCError(e));
            }

            thread::sleep(poll_interval);
        }
    })?;

    Ok(())
}
