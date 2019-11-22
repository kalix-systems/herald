use super::*;

pub(super) fn gc_handler(update: gc::GCUpdate) {
    use crate::imp::messages::{shared::MsgUpdate, Messages};
    use gc::GCUpdate::*;
    use heraldcore::errors::HErr;
    match update {
        StaleConversations(convs) => {
            for (cid, mids) in convs {
                push_err!(
                    Messages::push(cid, MsgUpdate::ExpiredMessages(mids)),
                    "Couldn't expire messages"
                );
            }
        }
        GCError(e) => {
            push_err!(Err::<(), HErr>(e), "Error deleting expired messages");
        }
    }
}
