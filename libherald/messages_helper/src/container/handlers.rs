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

    pub fn handle_reaction<F: FnMut(usize)>(
        &self,
        mid: MsgId,
        reactionary: UserId,
        reaction: heraldcore::message::ReactContent,
        remove: bool,
        mut data_changed: F,
    ) -> Option<()> {
        update(&mid, move |data| {
            if data.reactions.is_none() {
                data.reactions = Default::default();
            }

            if let Some(r) = data.reactions.as_mut() {
                if remove {
                    r.remove(reaction, reactionary);
                } else {
                    r.add(reaction, reactionary);
                }
            }
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
