extern crate emoji_utils;
use crate::interface::{
    EmojiPickerEmitter as Emitter, EmojiPickerList as List, EmojiPickerTrait as Interface,
};

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

    fn activities_index(&self) -> u32 {
        emoji_utils::ACTIVITIES as u32
    }

    fn body_index(&self) -> u32 {
        emoji_utils::PEOPLE_BODY as u32
    }

    fn flags_index(&self) -> u32 {
        emoji_utils::FLAGS as u32
    }

    fn food_index(&self) -> u32 {
        emoji_utils::FOOD_DRINK as u32
    }

    fn locations_index(&self) -> u32 {
        emoji_utils::TRAVEL_PLACES as u32
    }

    fn nature_index(&self) -> u32 {
        emoji_utils::ANIMALS_NATURE as u32
    }

    fn smileys_index(&self) -> u32 {
        emoji_utils::SMILEYS_EMOTION as u32
    }

    fn symbols_index(&self) -> u32 {
        emoji_utils::SYMBOLS as u32
    }

    fn objects_index(&self) -> u32 {
        emoji_utils::OBJECTS as u32
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn set_search_string(
        &mut self,
        search_string: String,
    ) {
        self.list.begin_reset_model();
        self.inner.search(search_string);
        self.list.end_reset_model();
    }

    fn clear_search(&mut self) {
        self.list.begin_reset_model();
        self.inner.clear_search();
        self.list.end_reset_model();
    }

    fn row_count(&self) -> usize {
        self.inner
            .current_emojis
            .as_ref()
            .map(Vec::len)
            .unwrap_or(0)
    }

    fn emoji(
        &self,
        index: usize,
    ) -> &str {
        self.inner
            .current_emojis
            .as_ref()
            .and_then(|list| list.get(index)?.emoji.into())
            .unwrap_or("")
    }

    fn skintone_modifier(
        &self,
        index: usize,
    ) -> bool {
        self.inner
            .current_emojis
            .as_ref()
            .and_then(|list| list.get(index)?.skintone_modifier.into())
            .unwrap_or(false)
    }
}
