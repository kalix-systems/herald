use super::*;

impl Messages {
    pub(crate) fn is_tail_(
        &self,
        index: usize,
    ) -> Option<bool> {
        if self.container.is_empty() {
            return None;
        }

        // Case where message is last message in conversation
        if index == 0 {
            return Some(true);
        }

        // other cases
        self.container
            .same_flurry(index, index - 1)
            .map(std::ops::Not::not)
    }

    pub(crate) fn is_head_(
        &self,
        index: usize,
    ) -> Option<bool> {
        if self.container.is_empty() {
            return None;
        }

        // Case where message is first message in conversation
        if index + 1 == self.container.len() {
            return Some(true);
        }

        // other cases
        self.container
            .same_flurry(index, index + 1)
            .map(std::ops::Not::not)
    }
}
