use crate::{ffi, interface::*, push_err, ret_err, ret_none, shared::SingletonBus};
use crate::{
    imp::{
        conversations::{shared::ConvUpdate, Conversations},
        users::shared::user_in_cache,
    },
    spawn,
};
use herald_common::UserId;
use heraldcore::conversation::{start, ConversationBuilder as Inner};
use std::convert::TryInto;

type Emitter = ConversationBuilderEmitter;
type List = ConversationBuilderList;

/// A builder for conversations
pub struct ConversationBuilder {
    emit: Emitter,
    model: List,
    inner: Inner,
    local_id: Option<UserId>,
}

impl ConversationBuilderTrait for ConversationBuilder {
    fn new(emit: Emitter, model: List) -> Self {
        Self {
            emit,
            model,
            local_id: push_err!(heraldcore::config::id(), "Failed to get local id"),
            inner: Inner::new(),
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn add_member(&mut self, user_id: ffi::UserId) -> bool {
        let local_id = ret_none!(self.local_id, false);

        let user_id: UserId = ret_err!(user_id.as_str().try_into(), false);

        // don't allow users to add themselves
        if user_id == local_id {
            return true;
        }

        // You should not be able to add users
        // that are unknown to you.
        if !user_in_cache(&user_id) || self.inner.has_member(&user_id) {
            return false;
        }

        let len = self.inner.members().len();
        self.model.begin_insert_rows(len, len);
        self.inner.add_member(user_id);
        self.model.end_insert_rows();

        true
    }

    fn remove_member_by_id(&mut self, user_id: ffi::UserId) -> bool {
        let user_id: UserId = ret_err!(user_id.as_str().try_into(), false);

        if !self.inner.has_member(&user_id) {
            return false;
        }

        let ix = ret_none!(
            self.inner.members().iter().position(|uid| uid == &user_id),
            false
        );

        self.model.begin_remove_rows(ix, ix);
        self.inner.remove_member_by_index(ix);
        self.model.end_remove_rows();

        true
    }

    fn remove_member_by_index(&mut self, index: u64) -> bool {
        let ix = index as usize;

        if ix >= self.inner.members().len() {
            return false;
        }

        self.model.begin_remove_rows(ix, ix);
        self.inner.remove_member_by_index(ix);
        self.model.end_remove_rows();

        true
    }

    fn remove_last(&mut self) {
        let len = self.inner.members().len();
        let pos = len.saturating_sub(1);

        if len != 0 {
            self.model.begin_remove_rows(pos, pos);
            self.inner.remove_member_by_index(pos);
            self.model.end_remove_rows();
        }
    }

    fn finalize(&mut self) {
        let inner = std::mem::replace(&mut self.inner, Default::default());

        spawn!({
            let conv = ret_err!(inner.add());

            // send update to Conversations list
            ret_err!(Conversations::push(ConvUpdate::BuilderFinished(
                conv.meta.clone()
            )));

            ret_err!(start(conv));
        });
    }

    fn set_title(&mut self, title: String) {
        self.inner.title(title);
    }

    fn picture(&self) -> Option<&str> {
        Some(self.inner.picture.as_ref()?.as_str())
    }

    fn set_picture(&mut self, picture: Option<String>) {
        self.inner.picture = picture.and_then(crate::utils::strip_qrc);
        self.emit.picture_changed();
    }

    fn row_count(&self) -> usize {
        self.inner.members().len()
    }

    fn member_id(&self, index: usize) -> ffi::UserIdRef {
        ret_none!(self.inner.members().get(index), "").as_str()
    }
}
