use crate::{
    ffi,
    interface::{UsersSearchEmitter as Emitter, UsersSearchList as List, UsersSearchTrait},
    ret_err,
    users::shared::{color, get_user, name, profile_picture, user_ids},
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

    /// Returns users name
    fn name(
        &self,
        row_index: usize,
    ) -> Option<String> {
        let uid = &self.list.get(row_index).as_ref()?.id;

        Some(name(uid).unwrap_or_else(|| uid.to_string()))
    }

    /// Returns profile picture
    fn profile_picture(
        &self,
        row_index: usize,
    ) -> Option<String> {
        let uid = &self.list.get(row_index)?.id;
        profile_picture(&uid)
    }

    /// Returns user's color
    fn color(
        &self,
        row_index: usize,
    ) -> Option<u32> {
        let uid = self.list.get(row_index)?.id;
        Some(color(&uid).unwrap_or(0))
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
                let pattern = ret_err!(SearchPattern::new_normal(pattern));
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
            match get_user(&user.id) {
                Some(inner) => {
                    user.matched = inner.matches(filter);

                    self.model.data_changed(ix, ix);
                }
                None => {
                    continue;
                }
            }
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
