use super::*;
use crate::{shared::AddressedBus, spawn};
use std::ops::Not;

#[derive(Default)]
/// A container type for messages backed by an RRB-tree vector
/// and a hash map.
pub struct Container {
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

    #[allow(unused)]
    pub(super) fn contains(&self, msg_id: &MsgId) -> bool {
        self.map.contains_key(msg_id)
    }

    pub(super) fn get(&self, ix: usize) -> Option<&Message> {
        self.list.get(ix)
    }

    pub(super) fn get_data_mut(&mut self, msg_id: &MsgId) -> Option<&mut MsgData> {
        self.map.get_mut(msg_id)
    }

    pub(super) fn get_data(&self, msg_id: &MsgId) -> Option<&MsgData> {
        self.map.get(msg_id)
    }

    pub(super) fn fill(cid: ConversationId) {
        spawn!({
            let (list, map): (Vector<Message>, HashMap<MsgId, MsgData>) =
                ret_err!(conversation::conversation_messages(&cid))
                    .into_iter()
                    .map(|m| {
                        let mid = m.message_id;
                        let (message, data) = Message::split_msg(m, SaveStatus::Saved);

                        (message, (mid, data))
                    })
                    .unzip();

            ret_err!(Messages::push(
                cid,
                MsgUpdate::Container(Self { list, map })
            ));
        });
    }

    pub(super) fn last_msg(&self) -> Option<&MsgData> {
        let mid = self.list.last()?.msg_id;
        self.map.get(&mid)
    }

    pub(super) fn msg_data(&self, index: usize) -> Option<&MsgData> {
        let msg = self.list.get(index);
        self.map.get(&msg?.msg_id)
    }

    #[allow(unused)]
    pub(super) fn msg_data_mut(&mut self, index: usize) -> Option<&mut MsgData> {
        let msg = self.list.get(index);
        self.map.get_mut(&msg?.msg_id)
    }

    pub(super) fn last(&self) -> Option<&Message> {
        self.list.last()
    }

    pub(super) fn index_of(&self, msg: &Message) -> Option<usize> {
        self.list.binary_search(&msg).ok()
    }

    pub(super) fn index_by_id(&self, msg_id: MsgId) -> Option<usize> {
        let m = Message::from_msg_id(msg_id, &self)?;

        self.list.binary_search(&m).ok()
    }

    /// Removes the item from the container. *Does not modify disk storage*.
    pub(super) fn mem_remove(&mut self, ix: usize) -> Option<MsgId> {
        if ix >= self.len() {
            return None;
        }

        let msg = self.list.remove(ix);
        self.map.remove(&msg.msg_id)?;

        Some(msg.msg_id)
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
        search: &SearchState,
        model: &mut List,
        emit: &mut Emitter,
    ) -> Option<Vec<Match>> {
        if search.active.not() || search.pattern.raw().is_empty() {
            return None;
        }

        let pattern = &search.pattern;

        let mut matches: Vec<Match> = Vec::new();

        for (ix, msg) in self.list.iter().enumerate() {
            let data = self.map.get_mut(&msg.msg_id)?;

            let old_status = data.match_status;
            let matched = data.matches(pattern);

            data.match_status = if matched {
                MatchStatus::Matched
            } else {
                MatchStatus::NotMatched
            };

            if old_status != data.match_status {
                model.data_changed(ix, ix);
            }

            if !matched {
                continue;
            };

            matches.push(Match(*msg))
        }

        emit.search_num_matches_changed();

        Some(matches)
    }

    pub(super) fn clear_search(&mut self, model: &mut List) -> Option<()> {
        for (ix, Message { msg_id, .. }) in self.list.iter().enumerate() {
            let data = self.map.get_mut(&msg_id)?;

            if data.match_status.is_match() {
                data.match_status = MatchStatus::NotMatched;
                model.data_changed(ix, ix);
            }
        }

        Some(())
    }

    pub(super) fn handle_receipt(
        &mut self,
        mid: MsgId,
        status: MessageReceiptStatus,
        recipient: UserId,
        model: &mut List,
    ) -> Result<(), HErr> {
        let msg: &mut MsgData = match self.map.get_mut(&mid) {
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

        msg.receipts
            .entry(recipient)
            .and_modify(|v| {
                if *v < status {
                    *v = status
                }
            })
            .or_insert(status);

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
            // it's probably very recent
            .rposition(|m| m.msg_id == mid)?;

        model.data_changed(ix, ix);

        Some(())
    }
}
