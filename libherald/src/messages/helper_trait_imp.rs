use heraldcore::types::ConversationId;
use messages_helper::types::{MessageEmit, MessageModel};

macro_rules! imp {
    ($($name: ident),*) => {
       $(fn $name(&mut self) {
           self.$name()
       })*
    }
}

impl MessageEmit for crate::interface::MessagesEmitter {
    imp!(
        search_num_matches_changed,
        search_pattern_changed,
        search_regex_changed,
        search_index_changed
    );

    fn last_changed(
        &mut self,
        cid: ConversationId,
    ) {
        self.last_msg_digest_changed();
        crate::conversation_content::new_activity(cid);
    }
}

impl MessageModel for crate::interface::MessagesList {
    fn data_changed(
        &mut self,
        a: usize,
        b: usize,
    ) {
        self.data_changed(a, b)
    }

    fn begin_remove_rows(
        &mut self,
        a: usize,
        b: usize,
    ) {
        self.begin_remove_rows(a, b)
    }

    fn end_remove_rows(&mut self) {
        self.end_remove_rows()
    }

    fn begin_insert_rows(
        &mut self,
        a: usize,
        b: usize,
    ) {
        self.begin_insert_rows(a, b)
    }

    fn end_insert_rows(&mut self) {
        self.end_insert_rows()
    }
}
