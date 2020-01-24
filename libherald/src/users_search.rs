use crate::{
    err, ffi,
    interface::{UsersSearchEmitter as Emitter, UsersSearchList as List, UsersSearchTrait},
    users::shared::user_ids,
};
use herald_common::UserId;
use search_pattern::SearchPattern;

struct User {
    id: UserId,
    matched: bool,
    selected: bool,
}

/// A wrapper around a vector of `User`s.
pub struct UsersSearch {
    emit: Emitter,
    model: List,
    filter: Option<SearchPattern>,
    list: Vec<User>,
}

impl UsersSearchTrait for UsersSearch {
    fn new(
        emit: Emitter,
        model: List,
    ) -> Self {
        Self {
            emit,
            model,
            filter: None,
            list: user_ids()
                .into_iter()
                .map(|id| User {
                    id,
                    matched: false,
                    selected: false,
                })
                .collect(),
        }
    }

    /// Returns user id.
    fn user_id(
        &self,
        row_index: usize,
    ) -> Option<ffi::UserIdRef> {
        Some(self.list.get(row_index)?.id.as_str())
    }

    fn selected(
        &self,
        row_index: usize,
    ) -> bool {
        self.list
            .get(row_index)
            .map(|u| u.selected)
            .unwrap_or(false)
    }

    fn set_selected(
        &mut self,
        row_index: usize,
        selected: bool,
    ) -> bool {
        if let Some(user) = self.list.get_mut(row_index) {
            user.selected = selected;
            self.clear_filter();
            return true;
        }
        return false;
    }

    fn matched(
        &self,
        row_index: usize,
    ) -> bool {
        self.list.get(row_index).map(|u| u.matched).unwrap_or(false)
    }

    fn clear_filter(&mut self) {
        self.filter = None;
        for user in self.list.iter_mut() {
            user.matched = false;
        }
        self.model
            .data_changed(0, self.list.len().saturating_sub(1));

        self.emit.filter_changed();
    }

    fn filter(&self) -> Option<&str> {
        Some(self.filter.as_ref()?.raw())
    }

    fn set_filter(
        &mut self,
        pattern: Option<String>,
    ) {
        match pattern {
            Some(pattern) => {
                if pattern.is_empty() {
                    self.clear_filter();
                    return;
                }
                let pattern = err!(SearchPattern::new_normal(pattern));
                self.filter = Some(pattern);
                self.inner_filter();
                self.emit.filter_changed();
            }
            None => self.clear_filter(),
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn refresh(&mut self) {
        self.model.begin_reset_model();
        self.filter = None;
        self.list = users();
        self.model.end_reset_model();
    }
}

impl UsersSearch {
    fn inner_filter(&mut self) -> Option<()> {
        let filter = &self.filter.as_ref()?;
        for (ix, user) in self.list.iter_mut().enumerate() {
            user.matched = crate::users::shared::matches(&user.id, filter);
            self.model.data_changed(ix, ix);
        }

        Some(())
    }
}

fn users() -> Vec<User> {
    user_ids()
        .into_iter()
        .map(|id| User {
            id,
            matched: false,
            selected: false,
        })
        .collect()
}
