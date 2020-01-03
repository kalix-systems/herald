use messages_helper::types::{MessageEmit, MessageModel};

impl MessageEmit for crate::interface::MessagesEmitter {
    fn search_num_matches_changed(&mut self) {
        self.search_num_matches_changed();
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
