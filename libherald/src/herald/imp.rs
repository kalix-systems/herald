use super::*;
use crate::content_push;

pub(crate) struct LoadProps {
    pub(crate) config: Config,
    pub(crate) conversation_builder: ConversationBuilder,
    pub(crate) conversations: Conversations,
    pub(crate) users: Users,
}

impl LoadProps {
    pub(super) fn setup(&mut self) {
        imp::start_gc();

        push_err!(self.config.try_load(), "Couldn't load Config");

        if let Some(id) = self.config.local_id() {
            self.conversation_builder.set_local_id(id);

            push_err!(self.conversations.try_load(), "Couldn't load Conversations");
            push_err!(self.users.try_load(), "Couldn't load Users");
        }
    }
}

fn gc_handler(update: gc::GCUpdate) {
    use crate::messages::MsgUpdate;
    use gc::GCUpdate::*;
    use heraldcore::errors::HErr;
    match update {
        StaleConversations(convs) => {
            for (cid, mids) in convs {
                push_err!(
                    content_push(cid, MsgUpdate::ExpiredMessages(mids)),
                    "Couldn't expire messages"
                );
            }
        }
        GCError(e) => {
            push_err!(Err::<(), HErr>(e), "Error deleting expired messages");
        }
    }
}

pub(super) fn start_gc() {
    // If this fails, it's because a thread couldn't be spawned.
    // This implies the OS is in a very bad place.
    push_err!(
        gc::init(move |update| {
            gc_handler(update);
        }),
        "Couldn't start GC thread"
    );
}
