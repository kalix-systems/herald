use crate::interface::{
    EmojiPickerEmitter as Emitter, EmojiPickerList as List, EmojiPickerTrait as Interface,
};
mod picker_struct;
use picker_struct::EMOJI_DATA;

/// The underlying struct of the emoji keyboard
pub struct EmojiPicker {
    emit: Emitter,
    _list: List,
}

impl Interface for EmojiPicker {
    fn new(
        emit: Emitter,
        model: List,
    ) -> Self {
        EmojiPicker { emit, _list: model }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn search_string(&self) -> Option<&str> {
        None
    }

    fn set_search_string(
        &mut self,
        value: Option<String>,
    ) {
        if let Some(search_string) = value {}
    }

    fn clear_search(&mut self) {}

    fn row_count(&self) -> usize {
        EMOJI_DATA.len()
    }

    fn fetch_more(&mut self) {
        //never called
    }

    fn emoji(
        &self,
        index: usize,
    ) -> String {
        String::from(EMOJI_DATA[index].emoji)
    }
}
