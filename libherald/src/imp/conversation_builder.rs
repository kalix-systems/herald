use crate::imp::{
    conversations::{shared::ConvUpdates, Conversations},
    users::shared::user_in_cache,
};
use crate::{bounds_chk, ffi, interface::*, ret_err, ret_none, shared::SingletonBus};
use herald_common::UserId;
use heraldcore::abort_err;
use std::convert::TryInto;

type Emitter = ConversationBuilderEmitter;
type List = ConversationBuilderList;

/// A builder for conversations
pub struct ConversationBuilder {
    emit: Emitter,
    model: List,
    list: Vec<UserId>,
    title: Option<String>,
    local_id: UserId,
    picture: Option<String>,
}

impl ConversationBuilderTrait for ConversationBuilder {
    fn new(emit: Emitter, model: List) -> Self {
        Self {
            emit,
            model,
            list: Vec::new(),
            title: None,
            local_id: abort_err!(heraldcore::config::Config::static_id()),
            picture: None,
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn add_member(&mut self, user_id: ffi::UserId) -> bool {
        let user_id: UserId = ret_err!(user_id.as_str().try_into(), false);

        // don't allow users to add themselves
        if user_id == self.local_id {
            return true;
        }

        // Avoid redundant inserts
        // This is linear time, yes, but
        // but n is probaby small.
        if self.list.contains(&user_id) {
            return true;
        }

        // You should not be able to add users
        // that you don't have as contacts.
        if !user_in_cache(&user_id) {
            return false;
        }

        self.model
            .begin_insert_rows(self.list.len(), self.list.len());
        self.list.push(user_id);
        self.model.end_insert_rows();

        true
    }

    fn remove_member_by_id(&mut self, user_id: ffi::UserId) -> bool {
        let user_id: UserId = ret_err!(user_id.as_str().try_into(), false);

        let ix = ret_none!(self.list.iter().position(|uid| uid == &user_id), false);

        self.model.begin_remove_rows(ix, ix);
        self.list.remove(ix);
        self.model.end_remove_rows();

        true
    }

    fn remove_member_by_index(&mut self, index: u64) -> bool {
        let ix = index as usize;

        bounds_chk!(self, ix, false);

        self.model.begin_remove_rows(ix, ix);
        self.list.remove(ix);
        self.model.end_remove_rows();

        true
    }

    fn remove_last(&mut self) {
        self.model.begin_remove_rows(
            self.list.len().saturating_sub(1),
            self.list.len().saturating_sub(1),
        );
        self.list.pop();
        self.model.end_remove_rows();
    }

    fn finalize(&mut self) {
        self.list.push(self.local_id);

        let list = std::mem::replace(&mut self.list, vec![]);
        let title = self.title.take();
        let picture = self.picture.take();

        ret_err!(std::thread::Builder::new().spawn(move || {
            let cid = ret_err!(heraldcore::network::start_conversation(
                &list, title, picture
            ));

            // send update to Conversations list
            ret_err!(Conversations::push(ConvUpdates::BuilderFinished(cid)));
        }));
    }

    fn set_title(&mut self, title: String) {
        self.title.replace(title);
    }

    fn picture(&self) -> Option<&str> {
        Some(self.picture.as_ref()?.as_str())
    }

    fn set_picture(&mut self, picture: Option<String>) {
        self.picture = picture.map(crate::utils::strip_qrc);
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn member_id(&self, index: usize) -> ffi::UserIdRef {
        ret_none!(self.list.get(index), "").as_str()
    }
}
