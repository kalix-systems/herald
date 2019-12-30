use crate::{
    conversations::shared::GlobalConvUpdate, err, ffi, interface::*, none, push, spawn,
    users::shared::user_in_cache,
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
    fn new(
        emit: Emitter,
        model: List,
    ) -> Self {
        Self {
            emit,
            model,
            local_id: None,
            inner: Inner::new(),
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn add_member(
        &mut self,
        user_id: ffi::UserId,
    ) -> bool {
        let local_id = none!(self.local_id, false);

        let user_id: UserId = err!(user_id.as_str().try_into(), false);

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

    fn remove_member_by_id(
        &mut self,
        user_id: ffi::UserId,
    ) -> bool {
        let user_id: UserId = err!(user_id.as_str().try_into(), false);

        if !self.inner.has_member(&user_id) {
            return false;
        }

        let ix = none!(
            self.inner.members().iter().position(|uid| uid == &user_id),
            false
        );

        self.model.begin_remove_rows(ix, ix);
        self.inner.remove_member_by_index(ix);
        self.model.end_remove_rows();

        true
    }

    fn remove_member_by_index(
        &mut self,
        index: u64,
    ) -> bool {
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
        self.model.begin_reset_model();
        let inner = std::mem::replace(&mut self.inner, Default::default());
        self.model.end_reset_model();

        spawn!({
            let conv = err!(inner.add());

            // send update to Conversations list
            push(GlobalConvUpdate::BuilderFinished(conv.meta.clone()));

            err!(start(conv));
        });
    }

    fn clear(&mut self) {
        self.model.begin_reset_model();
        self.inner = Default::default();
        self.model.end_reset_model();
    }

    fn set_title(
        &mut self,
        title: String,
    ) {
        self.inner.title(title);
    }

    fn picture(&self) -> Option<&str> {
        Some(self.inner.tagged_picture.as_ref()?.path.as_str())
    }

    fn set_profile_picture(
        &mut self,
        picture_json: String,
    ) {
        self.inner.tagged_picture =
            heraldcore::image_utils::ProfilePicture::from_json_string(picture_json);
        self.emit.picture_changed();
    }

    fn row_count(&self) -> usize {
        self.inner.members().len()
    }

    fn member_id(
        &self,
        index: usize,
    ) -> ffi::UserIdRef {
        none!(self.inner.members().get(index), "").as_str()
    }

    fn member_color(
        &self,
        index: usize,
    ) -> u32 {
        self.inner
            .members()
            .get(index)
            .and_then(crate::users::shared::color)
            .unwrap_or(0)
    }

    fn member_name(
        &self,
        index: usize,
    ) -> String {
        self.inner
            .members()
            .get(index)
            .and_then(crate::users::shared::name)
            .unwrap_or_default()
    }

    fn member_profile_picture(
        &self,
        index: usize,
    ) -> String {
        self.inner
            .members()
            .get(index)
            .and_then(crate::users::shared::profile_picture)
            .unwrap_or_default()
    }
}

impl ConversationBuilder {
    pub(crate) fn set_local_id(
        &mut self,
        id: UserId,
    ) {
        self.local_id.replace(id);
    }
}
