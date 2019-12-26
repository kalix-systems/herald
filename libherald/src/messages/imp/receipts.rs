use super::*;

impl Messages {
    pub(crate) fn receipt_status_(
        &self,
        index: usize,
    ) -> Option<u32> {
        let local_id = self.local_id?;

        Some(
            self.container
                .access_by_index(index, |data| {
                    let author = data.author;
                    data.receipts
                        .iter()
                        .filter(|(k, _)| k != &&local_id && k != &&author)
                        .map(|(_, r)| *r as u32)
                        .max()
                })?
                .unwrap_or(MessageReceiptStatus::Nil as u32),
        )
    }

    pub(crate) fn last_status_(&self) -> Option<u32> {
        self.container
            .last_msg()?
            .receipts
            .values()
            .max()
            .map(|status| *status as u32)
    }

    pub(crate) fn user_receipts_(
        &self,
        index: usize,
    ) -> Option<String> {
        let receipts = self
            .container
            .access_by_index(index, |data| data.receipts.clone())?
            .into_iter()
            .filter(|(u, _)| Some(u) != self.local_id.as_ref())
            .map(|(userid, receipt)| (userid.to_string(), json::JsonValue::from(receipt as u32)))
            .collect::<HashMap<String, json::JsonValue>>();

        json::JsonValue::from(receipts).dump().into()
    }

    pub(crate) fn mark_read_(
        &mut self,
        index: u64,
    ) {
        let index = index as usize;

        let local_id = none!(self.local_id);
        let msg_id = *none!(self.container.msg_id(index));
        let cid = none!(self.conversation_id);

        let updated = none!(self.container.update_by_index(index, |data| {
            let status = data.receipts.entry(local_id).or_default();

            match *status {
                MessageReceiptStatus::Read => false,
                _ => {
                    *status = MessageReceiptStatus::Read;
                    true
                }
            }
        }));

        if !updated {
            return;
        }

        self.model.data_changed(index, index);

        spawn!({
            err!(heraldcore::message::add_receipt(
                msg_id,
                local_id,
                MessageReceiptStatus::Read
            ));
            err!(heraldcore::network::send_read_receipt(cid, msg_id));
        });
    }
}
