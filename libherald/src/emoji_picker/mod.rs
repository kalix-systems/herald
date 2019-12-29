use crate::interface::{
    EmojiPickerEmitter as Emitter, EmojiPickerList as List, EmojiPickerTrait as Interface,
};
extern crate emoji_utils;
use emoji_utils::{EmojiUtil, Language};
/// The underlying struct of the emoji keyboard
pub struct EmojiPicker {
    emit: Emitter,
    list: List,
    inner: EmojiUtil,
}

impl Interface for EmojiPicker {
    fn new(
        emit: Emitter,
        model: List,
    ) -> Self {
        EmojiPicker {
            emit,
            list: model,
            inner: EmojiUtil::new(Language::En),
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn search_string(&self) -> Option<&str> {
        None
        //self.inner.search_string.as_deref()
    }

    fn set_search_string(
        &mut self,
        value: Option<String>,
    ) {
        if let Some(search_string) = value {
            self.list.begin_reset_model();
            self.inner.search(search_string);
            self.list.end_reset_model();
        }
    }

    fn clear_search(&mut self) {}

    fn row_count(&self) -> usize {
        if let Some(list) = &self.inner.current_emojis {
            list.len()
        } else {
            0
        }
    }

    fn emoji(
        &self,
        index: usize,
    ) -> String {
        if let Some(emoji_list) = &self.inner.current_emojis {
            String::from(emoji_list[index])
        } else {
            String::from("")
        }
    }
}
