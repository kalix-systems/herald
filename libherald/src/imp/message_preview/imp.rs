use super::*;
use crate::shared::AddressedBus;

impl MessagePreview {
    pub(super) fn fill(
        &mut self,
        mid: MsgId,
    ) {
        // This is for exception safety
        spawn!(if let Some(Message {
            time,
            body,
            author,
            has_attachments,
            ..
        }) = ret_err!(get_message_opt(&mid))
        {
            let update = shared::Update::Init {
                time: time.insertion,
                body,
                author,
                has_attachments,
            };
            ret_err!(MessagePreview::push(mid, update).map_err(|_| channel_send_err!()));
        });
    }

    pub(super) fn fetch_more_(&mut self) -> Option<()> {
        use shared::Update::*;

        let mid = self.msg_id?;
        let rx = shared::RXS.get(&mid)?;

        match rx.try_recv().ok()? {
            Init {
                time,
                body,
                author,
                has_attachments,
            } => {
                self.is_dangling = false;
                self.has_attachments = has_attachments;
                self.time.replace(time);
                self.author.replace(author);
                self.body = body;

                self.emit.message_id_changed();
                self.emit.msg_id_set_changed();
                self.emit.body_changed();
                self.emit.author_changed();
                self.emit.epoch_timestamp_ms_changed();
                self.emit.is_dangling_changed();

                if self.has_attachments {
                    self.emit.has_attachments_changed();
                }
            }
            SetDangling => {
                if self.body.is_some() {
                    self.body = None;
                    self.emit.body_changed();
                }

                if self.author.is_some() {
                    self.author = None;
                    self.emit.author_changed();
                }

                if self.time.is_some() {
                    self.time = None;
                    self.emit.epoch_timestamp_ms_changed();
                }

                if !self.is_dangling {
                    self.is_dangling = true;
                    self.emit.is_dangling_changed();
                }

                if self.has_attachments {
                    self.has_attachments = false;
                    self.emit.has_attachments_changed();
                }
            }
        }

        Some(())
    }
}

impl Drop for MessagePreview {
    fn drop(&mut self) {
        use shared::*;
        if let Some(mid) = self.msg_id {
            EMITTERS.remove(&mid);
            TXS.remove(&mid);
            RXS.remove(&mid);
        }
    }
}
