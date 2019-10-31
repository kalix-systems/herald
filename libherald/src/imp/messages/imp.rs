use super::*;
use std::ops::Drop;

impl Messages {
    pub(super) fn last_msg(&self) -> Option<&MsgData> {
        let mid = self.list.last()?.msg_id;
        self.map.get(&mid)
    }

    pub(super) fn msg_data(&self, index: usize) -> Option<&MsgData> {
        let msg = self.list.get(index);
        self.map.get(&msg?.msg_id)
    }

    pub(super) fn emit_last_changed(&mut self) {
        self.emit.last_author_changed();
        self.emit.last_body_changed();
        self.emit.last_epoch_timestamp_ms_changed();
        self.emit.last_status_changed();
    }

    pub(super) fn raw_list_remove(&mut self, ix: usize, id: &MsgId) {
        let len = self.list.len();

        let init_prev_state = if ix > 0 {
            (self.is_tail(ix - 1), self.is_head(ix - 1))
        } else {
            (None, None)
        };

        let init_succ_state = (self.is_tail(ix), self.is_head(ix));

        self.model.begin_remove_rows(ix, ix);
        self.list.remove(ix);
        self.map.remove(&id);
        self.model.end_remove_rows();

        if ix + 1 == len {
            self.emit.is_empty_changed();
        }

        if ix > 0 && init_prev_state != (self.is_head(ix - 1), self.is_tail(ix - 1)) {
            self.model.data_changed(ix - 1, ix - 1);
        }

        if ix + 1 < self.list.len()
            && init_succ_state != (self.is_head(ix - 1), self.is_tail(ix + 1))
        {
            self.model.data_changed(ix + 1, ix + 1);
        }

        self.emit.is_empty_changed();
    }

    pub(super) fn raw_insert(&mut self, msg: Msg, save_status: SaveStatus) -> Result<(), HErr> {
        let (message, data) = Message::split_msg(msg, save_status);

        let msg_id = message.msg_id;
        let cid = self.conversation_id.ok_or(NE!())?;

        let ix = if self
            .list
            .last()
            .map(|last| last.insertion_time)
            .unwrap_or(message.insertion_time)
            <= message.insertion_time
        {
            self.list.len()
        } else {
            match self.list.binary_search(&message) {
                Ok(_) => {
                    eprintln!(
                        "WARNING: tried to insert duplicate message at {file}:{line}:{col}",
                        file = file!(),
                        line = line!(),
                        col = column!()
                    );
                    return Ok(());
                }
                Err(ix) => ix,
            }
        };

        let init_prev_state = if ix > 0 { self.is_tail(ix - 1) } else { None };

        let init_succ_state = self.is_tail(ix);

        self.model.begin_insert_rows(ix, ix);
        self.list.insert(ix, message);
        self.map.insert(msg_id, data);
        self.model.end_insert_rows();

        if ix + 1 == self.list.len() {
            self.emit_last_changed();
        }

        if self.list.len() == 1 {
            self.emit.is_empty_changed();
        }

        if ix > 0 && init_prev_state != self.is_tail(ix - 1) {
            self.model.data_changed(ix - 1, ix - 1);
        }

        if ix + 1 < self.list.len() && init_succ_state != self.is_tail(ix + 1) {
            self.model.data_changed(ix + 1, ix + 1);
        }

        use crate::imp::conversations::{shared::*, Conversations};
        Conversations::push(ConvUpdates::NewActivity(cid))?;

        Ok(())
    }

    pub(super) fn index_of(&self, msg_id: MsgId) -> Option<usize> {
        let insertion_time = self.map.get(&msg_id)?.time.insertion;
        let m = Message {
            msg_id,
            insertion_time,
        };

        self.list.binary_search(&m).ok()
    }
}

impl Drop for Messages {
    fn drop(&mut self) {
        if let Some(cid) = self.conversation_id {
            EMITTERS.remove(&cid);
            TXS.remove(&cid);
            RXS.remove(&cid);
        }
    }
}
