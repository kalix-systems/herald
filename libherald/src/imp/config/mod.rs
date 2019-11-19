use crate::{ffi, interface::*, push_err, ret_err, ret_none};
use heraldcore::config::Config as Core;

/// Thin wrapper around heraldcore::config::Config,
/// with a field containing emitters for Qt.
pub struct Config {
    emit: ConfigEmitter,
    inner: Option<Core>,
}

// TODO this isn't exception safe
impl ConfigTrait for Config {
    /// Returns new Config. Will typically end up being called from C++
    fn new(emit: ConfigEmitter) -> Self {
        let inner = push_err!(Core::get(), "Couldn't fetch `Config`");
        Config { emit, inner }
    }

    /// UserId of the current user as an `&str`.
    fn config_id(&self) -> ffi::UserIdRef {
        ret_none!(self.inner.as_ref(), &ffi::NULL_USER_ID)
            .id
            .as_str()
    }

    /// Name of the current user
    fn name(&self) -> &str {
        ret_none!(self.inner.as_ref(), "").name.as_str()
    }

    /// Sets the name of the current user.
    fn set_name(&mut self, name: String) {
        let inner = ret_none!(self.inner.as_mut());

        let name = if name.is_empty() {
            inner.id.as_str().to_owned()
        } else {
            name
        };

        ret_err!(inner.set_name(name));

        self.emit.name_changed();
    }

    /// Returns the path to the current users profile picture, if it is set.
    /// Otherwise returns None.
    fn profile_picture(&self) -> Option<&str> {
        ret_none!(self.inner.as_ref(), None)
            .profile_picture
            .as_ref()
            .map(|s| s.as_str())
    }

    /// Returns id of the "note to self" conversation
    fn nts_conversation_id(&self) -> ffi::ConversationIdRef {
        ret_none!(self.inner.as_ref(), &ffi::NULL_CONV_ID)
            .nts_conversation
            .as_slice()
    }

    /// Sets the profile picture of the current user to the picture at the specified path.
    /// If `picture` is None, this clears the user's profile picture.
    fn set_profile_picture(&mut self, picture: Option<String>) {
        ret_err!(ret_none!(self.inner.as_mut())
            .set_profile_picture(picture.map(crate::utils::strip_qrc)));
        self.emit.profile_picture_changed();
    }
    /// Returns the color of the current user.
    fn color(&self) -> u32 {
        ret_none!(self.inner.as_ref(), 0).color
    }

    /// Sets the color of the current user.
    fn set_color(&mut self, color: u32) {
        ret_err!(ret_none!(self.inner.as_mut()).set_color(color));
        self.emit.color_changed();
    }

    /// Returns of the colorscheme of the current user.
    fn colorscheme(&self) -> u32 {
        ret_none!(self.inner.as_ref(), 0).colorscheme
    }

    /// Set the colorscheme of the current user.
    fn set_colorscheme(&mut self, colorscheme: u32) {
        ret_err!(ret_none!(self.inner.as_mut()).set_colorscheme(colorscheme));
        self.emit.colorscheme_changed();
    }

    fn emit(&mut self) -> &mut ConfigEmitter {
        &mut self.emit
    }
}
