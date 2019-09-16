use crate::{interface::*, ret_err};
use heraldcore::{
    abort_err,
    conversation::{ConversationMeta, Conversations as Core},
};

pub struct Conversations {
    emit: ConversationsEmitter,
    model: ConversationsList,
    list: Vec<ConversationMeta>,
    handle: Core,
}

impl ConversationsTrait for Conversations {
    fn new(emit: ConversationsEmitter, model: ConversationsList) -> Self {
        let handle = abort_err!(Core::new());
        let list = abort_err!(handle.all_meta());

        Self {
            emit,
            model,
            handle,
            list,
        }
    }

    fn emit(&mut self) -> &mut ConversationsEmitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn color(&self, index: usize) -> u32 {
        self.list[index].color
    }

    fn set_color(&mut self, index: usize, color: u32) -> bool {
        let meta = &mut self.list[index];
        ret_err!(self.handle.set_color(&meta.conversation_id, color), false);

        meta.color = color;
        true
    }

    fn conversation_id(&self, index: usize) -> &[u8] {
        self.list[index].conversation_id.as_slice()
    }

    fn muted(&self, index: usize) -> bool {
        self.list[index].muted
    }

    fn set_muted(&mut self, index: usize, muted: bool) -> bool {
        let meta = &mut self.list[index];
        ret_err!(self.handle.set_muted(&meta.conversation_id, muted), false);

        meta.muted = muted;
        true
    }

    fn picture(&self, index: usize) -> Option<&str> {
        self.list[index].picture.as_ref().map(|p| p.as_str())
    }

    fn set_picture(&mut self, index: usize, picture: Option<String>) -> bool {
        unimplemented!()
    }

    fn title(&self, index: usize) -> Option<&str> {
        self.list[index].title.as_ref().map(|t| t.as_str())
    }

    fn set_title(&mut self, index: usize, title: Option<String>) -> bool {
        let meta = &mut self.list[index];
        ret_err!(
            self.handle
                .set_title(&meta.conversation_id, title.as_ref().map(|t| t.as_str())),
            false
        );

        meta.title = title;
        true
    }
}
