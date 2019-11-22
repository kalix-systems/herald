use super::*;

impl Messages {
    pub(super) fn new_(
        emit: Emitter,
        model: List,
        builder: MessageBuilder,
    ) -> Self {
        Messages {
            model,
            emit,
            container: Container::default(),
            conversation_id: None,
            local_id: config::id().ok(),
            search: SearchState::new(),
            builder,
        }
    }

    pub(super) fn fetch_more_(&mut self) {
        let conv_id = ret_none!(self.conversation_id);

        let rx = match RXS.get(&conv_id) {
            Some(rx) => rx,
            // it's not a problem if the model doesn't have a receiver yet
            None => return,
        };

        for update in rx.try_iter() {
            match update {
                MsgUpdate::NewMsg(new) => {
                    new_msg_toast(&new);

                    ret_err!(self.insert_helper(*new, SaveStatus::Saved));
                }
                MsgUpdate::BuilderMsg(msg) => {
                    ret_err!(self.insert_helper(*msg, SaveStatus::Unsaved));
                }
                MsgUpdate::Receipt {
                    msg_id,
                    recipient,
                    status,
                } => {
                    ret_err!(container::handle_receipt(
                        &mut self.container,
                        msg_id,
                        status,
                        recipient,
                        &mut self.model
                    ));
                }
                MsgUpdate::StoreDone(mid) => {
                    ret_none!(container::handle_store_done(
                        &mut self.container,
                        mid,
                        &mut self.model
                    ));
                }

                MsgUpdate::ExpiredMessages(mids) => self.handle_expiration(mids),

                MsgUpdate::Container(container) => {
                    if container.is_empty() {
                        continue;
                    }

                    self.model
                        .begin_insert_rows(0, container.len().saturating_sub(1));
                    self.container = container;
                    self.model.end_insert_rows();
                    self.emit.is_empty_changed();
                    self.emit_last_changed();
                }
            }
        }
    }

    pub(super) fn receipt_status_(
        &self,
        index: usize,
    ) -> Option<u32> {
        Some(
            self.container
                .msg_data(index)?
                .receipts
                .values()
                .map(|r| *r as u32)
                .max()
                .unwrap_or(MessageReceiptStatus::NoAck as u32),
        )
    }

    pub(super) fn set_conversation_id_(
        &mut self,
        conversation_id: Option<ffi::ConversationIdRef>,
    ) {
        if let (Some(id), None) = (conversation_id, self.conversation_id) {
            let conversation_id = ret_err!(ConversationId::try_from(id));

            EMITTERS.insert(conversation_id, self.emit().clone());
            // remove left over channel from previous session
            RXS.remove(&conversation_id);
            TXS.remove(&conversation_id);

            self.conversation_id = Some(conversation_id);
            self.builder.set_conversation_id(conversation_id);
            self.emit.conversation_id_changed();

            container::fill(conversation_id);
        }
    }

    pub(super) fn last_author_(&self) -> Option<ffi::UserIdRef> {
        let last = self.container.last_msg()?;

        if last.author == self.local_id? {
            Some("You")
        } else {
            Some(last.author.as_str())
        }
    }

    pub(super) fn last_status_(&self) -> Option<u32> {
        self.container
            .last_msg()?
            .receipts
            .values()
            .max()
            .map(|status| *status as u32)
    }

    pub(super) fn index_by_id_(
        &self,
        msg_id: ffi::MsgIdRef,
    ) -> u64 {
        let ret_val = std::u32::MAX as u64;

        let msg_id = ret_err!(msg_id.try_into(), ret_val);

        ret_none!(self.container.index_by_id(msg_id), ret_val) as u64
    }

    pub(super) fn can_fetch_more_(&self) -> bool {
        let conv_id = match &self.conversation_id {
            Some(cid) => cid,
            None => return false,
        };

        let rx = match RXS.get(conv_id) {
            Some(rx) => rx,
            // it's not a problem if the model doesn't have a receiver yet
            None => return false,
        };

        !rx.is_empty()
    }

    pub(super) fn is_tail_(
        &self,
        index: usize,
    ) -> Option<bool> {
        if self.container.is_empty() {
            return None;
        }

        // Case where message is last message in conversation
        if index == self.container.len().saturating_sub(1) {
            return Some(true);
        }

        // other cases
        let (msg, succ) = (
            self.container.msg_data(index)?,
            self.container.msg_data(index + 1)?,
        );

        Some(!msg.same_flurry(succ))
    }

    pub(super) fn is_head_(
        &self,
        index: usize,
    ) -> Option<bool> {
        if self.container.is_empty() {
            return None;
        }

        // Case where message is first message in conversation
        if index == 0 {
            return Some(true);
        }

        // other cases
        let (msg, prev) = (
            self.container.msg_data(index)?,
            self.container.msg_data(index - 1)?,
        );

        Some(!msg.same_flurry(prev))
    }

    pub(super) fn clear_conversation_history_(&mut self) -> bool {
        let id = ret_none!(self.conversation_id, false);

        spawn!(conversation::delete_conversation(&id), false);

        self.clear_search();
        self.model
            .begin_remove_rows(0, self.container.len().saturating_sub(1));
        self.container = Default::default();
        self.model.end_remove_rows();

        self.emit_last_changed();
        self.emit.is_empty_changed();
        true
    }

    pub(super) fn delete_message_(
        &mut self,
        index: u64,
    ) -> bool {
        let ix = index as usize;

        let id = ret_none!(self.container.get(ix), false).msg_id;

        self.remove_helper(id, ix);
        spawn!(message::delete_message(&id), false);

        true
    }

    pub(super) fn search_pattern_(&self) -> &str {
        self.search
            .pattern
            .as_ref()
            .map(SearchPattern::raw)
            .unwrap_or("")
    }

    pub(super) fn op_body_(
        &self,
        index: usize,
    ) -> Option<&str> {
        self.container
            .op_body(index)?
            .as_ref()
            .map(MessageBody::as_str)
    }

    pub(super) fn set_builder_op_msg_id_(
        &mut self,
        id: Option<ffi::MsgIdRef>,
    ) {
        use builder::OpChanged;

        match ret_err!(self.builder.set_op_id(id, &self.container)) {
            OpChanged::Changed => {
                self.emit.builder_op_msg_id_changed();
            }
            OpChanged::NotChanged => {}
        }
    }

    pub(super) fn set_search_hint_(
        &mut self,
        scroll_position: f32,
        scroll_height: f32,
    ) {
        if scroll_position.is_nan() || scroll_height.is_nan() {
            return;
        }

        let percentage = scroll_position + scroll_height / 2.0;
        self.search.start_hint(percentage, &self.container);
    }

    /// Turns search on or off
    pub(super) fn set_search_active_(
        &mut self,
        active: bool,
    ) {
        if !active {
            self.search.active = false;
            self.clear_search();
        } else if !self.search.active {
            self.search.active = true;
            self.emit.search_active_changed();
            self.search.set_matches(
                apply_search(
                    &mut self.container,
                    &self.search,
                    &mut self.model,
                    &mut self.emit,
                )
                .unwrap_or_default(),
                &mut self.emit,
            );
        }
    }

    /// Clears search
    pub(super) fn clear_search_(&mut self) {
        clear_search(&mut self.container, &mut self.model);
        ret_err!(self.search.clear_search(&mut self.emit));
    }

    pub(super) fn set_search_pattern_(
        &mut self,
        pattern: String,
    ) {
        if pattern.is_empty() {
            self.clear_search();
            return;
        }

        if ret_err!(self.search.set_pattern(pattern, &mut self.emit)).changed()
            && self.search.active
        {
            self.search.set_matches(
                container::apply_search(
                    &mut self.container,
                    &self.search,
                    &mut self.model,
                    &mut self.emit,
                )
                .unwrap_or_default(),
                &mut self.emit,
            );
        }
    }

    /// Sets search mode
    pub(super) fn set_search_regex_(
        &mut self,
        use_regex: bool,
    ) {
        if ret_err!(self.search.set_regex(use_regex, &mut self.emit)).changed() {
            self.search.set_matches(
                container::apply_search(
                    &mut self.container,
                    &self.search,
                    &mut self.model,
                    &mut self.emit,
                )
                .unwrap_or_default(),
                &mut self.emit,
            );
        }
    }
}
