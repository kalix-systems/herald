use crate::{
    err, ffi,
    interface::{UserEmitter as Emit, UserTrait as Interface},
    none, spawn,
    users::shared as data,
};
use herald_common::UserId;
use std::convert::TryInto;

mod shared;
pub(crate) use shared::user_push;

/// Handle for user information
pub struct User {
    emit: Emit,
    id: Option<UserId>,
}

impl Interface for User {
    fn new(emit: Emit) -> Self {
        User { emit, id: None }
    }

    fn emit(&mut self) -> &mut Emit {
        &mut self.emit
    }

    fn name(&self) -> String {
        let uid = &none!(self.id, "".to_owned());

        none!(data::name(uid), uid.to_string())
    }

    fn pairwise_conversation_id(&self) -> Vec<u8> {
        let uid = &none!(self.id, ffi::NULL_CONV_ID.to_vec());

        none!(data::pairwise_cid(uid), ffi::NULL_CONV_ID.to_vec()).to_vec()
    }

    fn profile_picture(&self) -> Option<String> {
        let uid = &self.id?;
        data::profile_picture(&uid)
    }

    fn user_color(&self) -> u32 {
        let uid = none!(self.id, 0);
        data::color(&uid).unwrap_or(0)
    }

    fn set_user_color(
        &mut self,
        color: u32,
    ) {
        let uid = none!(self.id);

        spawn!({
            use crate::conversations::shared::{ConvItemUpdate as C, ConvItemUpdateVariant as CV};

            err!(heraldcore::user::set_color(uid, color));

            let cid = none!(data::pairwise_cid(&uid));

            crate::push(C {
                cid,
                variant: CV::UserChanged,
            });
        });

        {
            let mut lock = data::user_data().write();
            let mut inner = none!(lock.get_mut(&uid));
            inner.color = color;
        }

        self.emit.user_color_changed();
    }

    fn user_id(&self) -> Option<ffi::UserIdRef> {
        self.id.as_ref().map(UserId::as_str)
    }

    fn set_user_id(
        &mut self,
        id: Option<ffi::UserId>,
    ) {
        if let (None, Some(new)) = (self.id, id) {
            let new: UserId = err!(new.as_str().try_into());
            self.register_user(new);
        }
    }
}
