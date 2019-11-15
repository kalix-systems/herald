use super::*;
use heraldcore::{channel_recv_err, channel_send_err};

#[derive(Default)]
pub(super) struct Container {
    list: Vector<Message>,
    map: HashMap<MsgId, MsgData>,
}

impl Container {
    pub(super) fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub(super) fn len(&self) -> usize {
        self.list.len()
    }

    pub(super) fn get(&self, ix: usize) -> Option<&Message> {
        self.list.get(ix)
    }

    pub(super) fn new(cid: ConversationId) -> Result<Self, HErr> {
        let (tx, rx) = crossbeam_channel::bounded(0);

        // for exception safety
        std::thread::Builder::new().spawn(move || {
            let (list, map): (Vector<Message>, HashMap<MsgId, MsgData>) =
                ret_err!(conversation::conversation_messages(&cid))
                    .into_iter()
                    .map(|m| {
                        let mid = m.message_id;
                        let (message, data) = Message::split_msg(m, SaveStatus::Saved);

                        (message, (mid, data))
                    })
                    .unzip();

            ret_err!(tx.send(Self { list, map }).map_err(|_| channel_send_err!()));
        })?;

        rx.recv_timeout(std::time::Duration::from_secs(1))
            .map_err(|_| channel_recv_err!())
    }

    pub(super) fn last_msg(&self) -> Option<&MsgData> {
        let mid = self.list.last()?.msg_id;
        self.map.get(&mid)
    }

    pub(super) fn msg_data(&self, index: usize) -> Option<&MsgData> {
        let msg = self.list.get(index);
        self.map.get(&msg?.msg_id)
    }

    pub(super) fn last(&self) -> Option<&Message> {
        self.list.last()
    }

    pub(super) fn index_of(&self, msg_id: MsgId) -> Option<usize> {
        let insertion_time = self.map.get(&msg_id)?.time.insertion;
        let m = Message {
            msg_id,
            insertion_time,
        };

        self.list.binary_search(&m).ok()
    }

    /// Removes the item from the container. *Does not modify disk storage*.
    pub(super) fn mem_remove(&mut self, ix: usize) -> Option<()> {
        if ix >= self.len() {
            return None;
        }

        let msg = self.list.remove(ix);
        self.map.remove(&msg.msg_id);

        Some(())
    }

    pub(super) fn binary_search(&self, msg: &Message) -> Result<usize, usize> {
        self.list.binary_search(msg)
    }

    pub(super) fn insert(&mut self, ix: usize, msg: Message, data: MsgData) {
        let mid = msg.msg_id;

        self.list.insert(ix, msg);
        self.map.insert(mid, data);
    }

    pub(super) fn apply_search(
        &mut self,
        search: &SearchMachine,
        model: &mut List,
        emit: &mut Emitter,
    ) -> Option<Vec<Match>> {
        if !search.active {
            return None;
        }

        let old_cnt = search.matches.len();

        let pattern = &search.pattern;

        // to help the borrow checker
        let map = &mut self.map;

        let matches: Vec<Match> = self
            .list
            .iter()
            .enumerate()
            .map(|(ix, Message { msg_id, .. })| (ix, msg_id))
            .filter_map(|(ix, mid)| {
                let data = map.get_mut(&mid)?;
                let matches = data.matches(pattern);
                data.matched = matches;
                if !matches {
                    return None;
                };

                Some(Match { ix })
            })
            .collect();

        if old_cnt != matches.len() {
            emit.search_num_matches_changed();
        }

        model.data_changed(0, self.list.len().saturating_sub(1));

        Some(matches)
    }

    pub(super) fn clear_search(&mut self, model: &mut List) {
        for data in self.map.values_mut() {
            data.matched = false;
        }

        model.data_changed(0, self.list.len().saturating_sub(1));
    }

    pub(super) fn handle_receipt(&mut self, mid: MsgId, model: &mut List) -> Result<(), HErr> {
        let mut msg = match self.map.get_mut(&mid) {
            None => {
                // This can (possibly) happen if the message
                // was deleted between the receipt
                // being received over the network
                // and this part of the code.
                return Ok(());
            }
            Some(msg) => msg,
        };

        // NOTE: If this fails, there is a bug somewhere
        // in libherald.
        //
        // It is probably trivial, but may reflect something
        // deeply wrong with our understanding of the program's
        // concurrency.
        let ix = self
            .list
            .iter()
            // search backwards,
            // it's probably fairly recent
            .rposition(|m| m.msg_id == mid)
            .ok_or(NE!())?;

        // TODO exception safety
        let receipts = message::get_message_receipts(&mid)?;
        msg.receipts = receipts;

        model.data_changed(ix, ix);

        Ok(())
    }

    pub(super) fn handle_store_done(&mut self, mid: MsgId, model: &mut List) -> Option<()> {
        let data = self.map.get_mut(&mid)?;

        data.save_status = SaveStatus::Saved;
        let ix = self
            .list
            .iter()
            // search backwards,
            // it's probably fairly recent
            .rposition(|m| m.msg_id == mid)?;

        model.data_changed(ix, ix);

        Some(())
    }
}
