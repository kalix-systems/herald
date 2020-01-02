use super::*;

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

pub(super) fn start_gc() {
    // If this fails, it's because a thread couldn't be spawned.
    // This implies the OS is in a very bad place.
    push_err!(gc::init(), "Couldn't start GC thread");
}
