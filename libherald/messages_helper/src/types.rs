pub use heraldcore::{message::*, types::MsgId};

pub trait MessageModel {
    fn entry_changed(
        &mut self,
        ix: usize,
    ) {
        self.data_changed(ix, ix);
    }

    fn data_changed(
        &mut self,
        a: usize,
        b: usize,
    );

    fn begin_remove_rows(
        &mut self,
        a: usize,
        b: usize,
    );

    fn end_remove_rows(&mut self);

    fn begin_insert_rows(
        &mut self,
        a: usize,
        b: usize,
    );

    fn end_insert_rows(&mut self);
}

pub trait MessageEmit {
    fn search_num_matches_changed(&mut self);
    fn search_pattern_changed(&mut self);
    fn search_regex_changed(&mut self);
    fn search_index_changed(&mut self);
    fn last_has_attachments_changed(&mut self);
    fn is_empty_changed(&mut self);
    fn last_changed(&mut self);
}

pub fn from_msg_id(msg_id: MsgId) -> Option<MessageMeta> {
    let insertion_time = crate::container::access(&msg_id, |m| m.time.insertion)?;

    Some(MessageMeta {
        msg_id,
        insertion_time,
        match_status: MatchStatus::NotMatched,
    })
}
