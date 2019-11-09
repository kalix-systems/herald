use crate::{ffi, interface::*, ret_err};
use heraldcore::{abort_err, config::Config as Core};

/// Thin wrapper around heraldcore::config::Config,
/// with a field containing emitters for Qt.
pub struct Config {
    emit: ConfigEmitter,
    inner: Core,
}

impl ConfigTrait for Config {
    /// Returns new Config. Will typically end up being called from C++
    fn new(emit: ConfigEmitter) -> Self {
        let inner = abort_err!(Core::get());
        Config { emit, inner }
    }

    /// UserId of the current user as an `&str`.
    fn config_id(&self) -> ffi::UserIdRef {
        self.inner.id.as_str()
    }

    /// Name of the current user, if one is set.
    fn name(&self) -> &str {
        self.inner.name.as_str()
    }

    /// Sets the name of the current user. If `name` is None, this
    /// clears the name.
    fn set_name(&mut self, name: String) {
        let name = if name.is_empty() {
            self.inner.id.as_str().to_owned()
        } else {
            name
        };

        ret_err!(self.inner.set_name(name));

        self.emit.name_changed();
    }

    /// Returns the path to the current users profile picture, if it is set.
    /// Otherwise returns None.
    fn profile_picture(&self) -> Option<&str> {
        self.inner.profile_picture.as_ref().map(|s| s.as_str())
    }

    /// Returns id of the "note to self" conversation
    fn nts_conversation_id(&self) -> ffi::ConversationIdRef {
        self.inner.nts_conversation.as_slice()
    }

    /// Sets the profile picture of the current user to the picture at the specified path.
    /// If `picture` is None, this clears the user's profile picture.
    fn set_profile_picture(&mut self, picture: Option<String>) {
        ret_err!(self
            .inner
            .set_profile_picture(picture.map(crate::utils::strip_qrc)));
        self.emit.profile_picture_changed();
    }
    /// Returns the color of the current user.
    fn color(&self) -> u32 {
        self.inner.color
    }

    /// Sets the color of the current user.
    fn set_color(&mut self, color: u32) {
        ret_err!(self.inner.set_color(color));
        self.emit.color_changed();
    }

    /// Returns of the colorscheme of the current user.
    fn colorscheme(&self) -> u32 {
        self.inner.colorscheme
    }

    /// Set the colorscheme of the current user.
    fn set_colorscheme(&mut self, colorscheme: u32) {
        match self.inner.set_colorscheme(colorscheme) {
            Ok(()) => self.emit.colorscheme_changed(),
            Err(e) => eprintln!("{}", e),
        }
    }

    fn emit(&mut self) -> &mut ConfigEmitter {
        &mut self.emit
    }
}
