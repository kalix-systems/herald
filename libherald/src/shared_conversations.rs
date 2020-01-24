use crate::{
    err, ffi,
    interface::{
        SharedConversationsEmitter as Emit, SharedConversationsList as List,
        SharedConversationsTrait as Interface,
    },
    spawn,
};
use crossbeam_channel::{bounded, Receiver};
use herald_common::UserId;
use heraldcore::types::ConversationId;
use std::convert::TryFrom;

/// Conversations shared with a user
pub struct SharedConversations {
    uid: Option<UserId>,
    inner: Vec<ConversationId>,
    model: List,
    emit: Emit,
    rx: Option<Receiver<Vec<ConversationId>>>,
}

impl Interface for SharedConversations {
    fn new(
        emit: Emit,
        model: List,
    ) -> Self {
        Self {
            uid: None,
            inner: vec![],
            model,
            emit,
            rx: None,
        }
    }

    fn emit(&mut self) -> &mut Emit {
        &mut self.emit
    }

    fn user_id(&self) -> Option<&str> {
        self.uid.as_ref()?.as_str().into()
    }

    fn set_user_id(
        &mut self,
        uid: Option<String>,
    ) {
        if let Some(uid) = uid {
            let uid = err!(UserId::try_from(uid.as_str()));
            self.uid = Some(uid);
            self.emit.user_id_changed();

            let (tx, rx) = bounded(1);
            self.rx.replace(rx);

            let mut emit = self.emit.clone();
            spawn!({
                let cids = err!(heraldcore::members::shared_conversations(&uid));
                drop(tx.send(cids));
                emit.try_load();
            });
        }
    }

    fn load(&mut self) {
        if let Some(rx) = self.rx.as_ref() {
            if let Ok(cids) = rx.recv() {
                self.model.begin_reset_model();
                self.inner = cids;
                self.model.end_reset_model();
            }
        }
    }

    fn row_count(&self) -> usize {
        self.inner.len()
    }

    fn conversation_id(
        &self,
        index: usize,
    ) -> ffi::ConversationIdRef {
        self.inner
            .get(index)
            .map(|c| c.as_slice())
            .unwrap_or(&ffi::NULL_CONV_ID)
    }
}
