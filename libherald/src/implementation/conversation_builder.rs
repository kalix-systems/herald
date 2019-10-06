use crate::{ffi, interface::*, ret_err, ret_none, shared::USER_DATA};
use herald_common::UserId;
use std::convert::TryInto;

type Emitter = ConversationBuilderEmitter;
type List = ConversationBuilderList;

/// A builder for conversations
pub struct ConversationBuilder {
    emit: Emitter,
    model: List,
    list: Vec<UserId>,
    title: Option<String>,
}

impl ConversationBuilderTrait for ConversationBuilder {
    fn new(emit: Emitter, model: List) -> Self {
        Self {
            emit,
            model,
            list: Vec::new(),
            title: None,
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn add_member(&mut self, user_id: ffi::UserId) -> bool {
        let user_id: UserId = ret_err!(user_id.as_str().try_into(), false);

        // Avoid redundant inserts
        // This is linear time, yes, but
        // but n is probaby small.
        if self.list.contains(&user_id) {
            return true;
        }

        // You should not be able to add users
        // that you don't have as contacts.
        if !USER_DATA.contains_key(&user_id) {
            return false;
        }

        self.model
            .begin_insert_rows(self.list.len(), self.list.len());
        self.list.push(user_id);
        self.model.end_insert_rows();

        true
    }

    fn finalize(&mut self) -> ffi::ConversationId {
        ret_err!(
            heraldcore::network::start_conversation(
                self.list.as_slice(),
                self.title.as_ref().map(String::as_str)
            ),
            ffi::NULL_CONV_ID.to_vec()
        )
        .to_vec()
    }

    fn set_title(&mut self, title: String) {
        self.title.replace(title);
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn display_name(&self, index: usize) -> String {
        let uid = ret_none!(self.list.get(index), "".to_owned());
        let inner = ret_none!(USER_DATA.get(uid), "".to_owned());
        match inner.name.as_ref() {
            Some(name) => name.clone(),
            None => inner.id.to_string(),
        }
    }

    fn color(&self, index: usize) -> u32 {
        let uid = ret_none!(self.list.get(index), 0);
        let inner = ret_none!(USER_DATA.get(uid), 0);

        inner.color
    }
}
