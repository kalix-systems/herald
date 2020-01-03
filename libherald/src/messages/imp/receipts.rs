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
        id: ffi::MsgIdRef,
    ) {
        let msg_id = err!(id.try_into());

        let local_id = none!(self.local_id);
        let cid = none!(self.conversation_id);

        let updated = none!(messages_helper::container::update(&msg_id, |data| {
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

        let index = none!(self.container.index_by_id(msg_id));
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
