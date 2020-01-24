use crate::{err, ffi, interface::*, none, spawn};
use heraldcore::{
    config::{self as core, Config as Core},
    conversation::ExpirationPeriod,
};

mod imp;

/// Thin wrapper around heraldcore::config::Config,
/// with a field containing emitters for Qt.
pub struct Config {
    emit: ConfigEmitter,
    pub(crate) inner: Option<Core>,
}

impl ConfigTrait for Config {
    fn new(emit: ConfigEmitter) -> Self {
        Config { emit, inner: None }
    }

    /// UserId of the current user as an `&str`.
    fn config_id(&self) -> ffi::UserIdRef {
        none!(self.inner.as_ref(), &ffi::NULL_USER_ID).id.as_str()
    }

    /// Returns id of the "note to self" conversation
    fn nts_conversation_id(&self) -> ffi::ConversationIdRef {
        none!(self.inner.as_ref(), &ffi::NULL_CONV_ID)
            .nts_conversation
            .as_slice()
    }

    /// Returns of the preferred expiration period of the current user.
    fn preferred_expiration(&self) -> u8 {
        none!(self.inner.as_ref(), 0).preferred_expiration as u8
    }

    /// Sets the name of the current user.
    fn set_name(
        &mut self,
        name: String,
    ) {
        let inner = none!(self.inner.as_mut());

        let name = if name.is_empty() {
            inner.id.as_str().to_owned()
        } else {
            name
        };

        {
            let name = name.clone();
            spawn!(core::set_name(name));
        }

        crate::user_push(
            inner.id,
            herald_user::UserChange::DisplayName(Some(name.clone())),
        );

        inner.name = name;
    }

    /// Set  the preferred expiration period of the current user.
    fn set_preferred_expiration(
        &mut self,
        period: u8,
    ) {
        let inner = none!(self.inner.as_mut());
        let period = ExpirationPeriod::from(period);
        spawn!(core::set_preferred_expiration(period));

        inner.preferred_expiration = period;
    }

    /// Sets the profile picture of the current user to the picture at the specified path.
    /// If `picture` is None, this clears the user's profile picture.
    fn set_profile_picture(
        &mut self,
        picture_json: String,
    ) {
        spawn!({
            let profile_picture =
                heraldcore::image_utils::ProfilePicture::from_json_string(picture_json);

            let path = err!(core::set_profile_picture(profile_picture).map(ConfUpdate::Picture));
            crate::push(path);
        });
    }

    fn emit(&mut self) -> &mut ConfigEmitter {
        &mut self.emit
    }
}

/// Config update
pub enum ConfUpdate {
    /// Profile picture changed
    Picture(Option<String>),
}

use crate::Update;
impl From<ConfUpdate> for Update {
    fn from(update: ConfUpdate) -> Update {
        Update::Conf(update)
    }
}
