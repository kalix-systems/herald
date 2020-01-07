use super::*;

impl Container {
    pub fn handle_receipt<M: MessageModel>(
        &self,
        mid: MsgId,
        status: MessageReceiptStatus,
        recipient: UserId,
        model: &mut M,
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

        model.entry_changed(ix);

        Some(())
    }

    pub fn handle_store_done<E: MessageEmit, M: MessageModel>(
        &mut self,
        mid: MsgId,
        meta: herald_attachments::AttachmentMeta,
        emit: &mut E,
        model: &mut M,
        conversation_id: ConversationId,
    ) -> Option<()> {
        if meta.is_empty() {
            return Some(());
        }

        {
            let meta = &meta;
            update(&mid, move |data| {
                if let Item::Plain(PlainItem {
                    ref mut attachments,
                    ..
                }) = data.content
                {
                    *attachments = meta.clone();
                }
            })?;
        }

        let ix = self
            .list
            .iter()
            // search backwards,
            // it's probably very recent
            .rposition(|m| m.msg_id == mid)?;

        model.entry_changed(ix);

        if ix == 0 {
            emit.last_changed(conversation_id, self.last().map(|m| m.msg_id));
        }

        Some(())
    }

    pub fn handle_send_done<M: MessageModel>(
        &self,
        mid: MsgId,
        model: &mut M,
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

        model.entry_changed(ix);

        Some(())
    }

    pub fn handle_reaction<M: MessageModel>(
        &self,
        mid: MsgId,
        reactionary: UserId,
        reaction: heraldcore::message::ReactContent,
        remove: bool,
        model: &mut M,
    ) -> Option<()> {
        update(&mid, move |data| {
            if data.reactions.is_none() {
                data.reactions.replace(Default::default());
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

        model.entry_changed(ix);

        Some(())
    }

    pub fn handle_expiration<E: MessageEmit, M: MessageModel, B: MessageBuilderHelper>(
        &mut self,
        mids: Vec<MsgId>,
        emit: &mut E,
        model: &mut M,
        search: &mut SearchState,
        builder: &mut B,
        cid: ConversationId,
    ) {
        for mid in mids {
            if let Some(ix) = self.index_by_id(mid) {
                self.remove_helper(mid, ix, emit, model, search, builder, cid);
            }
        }
    }
}
