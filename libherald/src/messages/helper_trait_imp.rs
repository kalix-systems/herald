use messages_helper::types::{MessageEmit, MessageModel};

impl MessageEmit for crate::interface::MessagesEmitter {
    fn search_num_matches_changed(&mut self) {
        self.search_num_matches_changed();
    }

    fn search_pattern_changed(&mut self) {
        self.search_pattern_changed();
    }

    fn search_regex_changed(&mut self) {
        self.search_regex_changed();
    }

    fn search_index_changed(&mut self) {
        self.search_index_changed();
    }

    fn last_has_attachments_changed(&mut self) {
        self.last_has_attachments_changed();
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
}
