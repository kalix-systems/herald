use super::*;

impl Container {
    pub fn handle_receipt<F: FnMut(usize)>(
        &self,
        mid: MsgId,
        status: MessageReceiptStatus,
        recipient: UserId,
        mut data_changed: F,
    ) -> Option<()> {
        let res = update(&mid, |msg| {
            msg.receipts
                .entry(recipient)
                .and_modify(|v| {
                    if *v < status {
                        *v = status
                    }
                })
                .or_insert(status);
        });

        if res.is_none() {
            return Some(());
        }

        let ix = self
            .list
            .iter()
            // search backwards,
            // it's probably fairly recent
            .rposition(|m| m.msg_id == mid)?;

        data_changed(ix);

        Some(())
    }

    pub fn handle_store_done<F: FnMut(usize)>(
        &self,
        mid: MsgId,
        meta: heraldcore::message::attachments::AttachmentMeta,
        mut data_changed: F,
    ) -> Option<()> {
        update(&mid, move |data| {
            data.attachments = meta;
        })?;

        let ix = self
            .list
            .iter()
            // search backwards,
            // it's probably very recent
            .rposition(|m| m.msg_id == mid)?;

        data_changed(ix);

        Some(())
    }

    pub fn handle_send_done<F: FnMut(usize)>(
        &self,
        mid: MsgId,
        mut data_changed: F,
    ) -> Option<()> {
        update(&mid, move |data| {
            data.send_status = heraldcore::message::MessageSendStatus::Ack;
        })?;

        let ix = self
            .list
            .iter()
            // search backwards,
            // it's probably very recent
            .rposition(|m| m.msg_id == mid)?;

        data_changed(ix);

        Some(())
    }
}
