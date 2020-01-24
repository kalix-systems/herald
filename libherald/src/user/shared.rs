use super::*;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::collections::HashMap;

static USER_EMIT: OnceCell<Mutex<HashMap<UserId, Emit>>> = OnceCell::new();

fn user_emit() -> &'static Mutex<HashMap<UserId, Emit>> {
    USER_EMIT.get_or_init(Default::default)
}

impl User {
    pub(super) fn register_user(
        &mut self,
        uid: UserId,
    ) {
        self.id.replace(uid);
        let mut emit_lock = user_emit().lock();
        emit_lock.insert(uid, self.emit.clone());
    }
}

pub(crate) fn user_push(
    uid: UserId,
    change: herald_user::UserChange,
) {
    use herald_user::UserChange as U;
    let mut data_lock = data::user_data().write();
    let mut emit_lock = user_emit().lock();

    let emit = none!(emit_lock.get_mut(&uid));

    match change {
        U::Color(color) => {
            data_lock.entry(uid).and_modify(|u| u.color = color);
            drop(data_lock);

            emit.user_color_changed();
        }
        U::Picture(picture) => {
            data_lock
                .entry(uid)
                .and_modify(|u| u.profile_picture = picture);
            drop(data_lock);

            emit.profile_picture_changed();
        }
        U::DisplayName(name) => {
            data_lock
                .entry(uid)
                .and_modify(|u| u.name = name.unwrap_or_else(|| u.id.to_string()));
            drop(data_lock);

            emit.name_changed();
        }
    }
}
