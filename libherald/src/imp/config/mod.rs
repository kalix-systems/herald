use crate::{ffi, interface::*, ret_err, ret_none, spawn};
use heraldcore::{
    config::{self as core, Config as Core},
    errors::HErr,
};

/// Thin wrapper around heraldcore::config::Config,
/// with a field containing emitters for Qt.
pub struct Config {
    emit: ConfigEmitter,
    inner: Option<Core>,
}

// TODO this isn't exception safe
impl ConfigTrait for Config {
    fn new(emit: ConfigEmitter) -> Self {
        Config { emit, inner: None }
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

    /// Returns the color of the current user.
    fn color(&self) -> u32 {
        ret_none!(self.inner.as_ref(), 0).color
    }

    /// Returns of the colorscheme of the current user.
    fn colorscheme(&self) -> u32 {
        ret_none!(self.inner.as_ref(), 0).colorscheme
    }

    /// Sets the color of the current user.
    fn set_color(
        &mut self,
        color: u32,
    ) {
        let inner = ret_none!(self.inner.as_mut());
        spawn!(core::set_color(color));
        inner.color = color;

        self.emit.color_changed();
    }

    /// Sets the name of the current user.
    fn set_name(
        &mut self,
        name: String,
    ) {
        let inner = ret_none!(self.inner.as_mut());

        let name = if name.is_empty() {
            inner.id.as_str().to_owned()
        } else {
            name
        };

        {
            let name = name.clone();
            spawn!(core::set_name(name));
        }

        inner.name = name;

        self.emit.name_changed();
    }

    /// Set the colorscheme of the current user.
    fn set_colorscheme(
        &mut self,
        colorscheme: u32,
    ) {
        let inner = ret_none!(self.inner.as_mut());

        spawn!(core::set_colorscheme(colorscheme));

        inner.colorscheme = colorscheme;

        self.emit.colorscheme_changed();
    }

    /// Sets the profile picture of the current user to the picture at the specified path.
    /// If `picture` is None, this clears the user's profile picture.
    fn set_profile_picture(
        &mut self,
        picture: Option<String>,
    ) {
        let inner = ret_none!(self.inner.as_mut());

        // TODO exception safety
        let picture = ret_err!(core::set_profile_picture(
            picture.and_then(crate::utils::strip_qrc)
        ));

        inner.profile_picture = picture;

        self.emit.profile_picture_changed();
    }

    fn emit(&mut self) -> &mut ConfigEmitter {
        &mut self.emit
    }
}

impl Config {
    pub(crate) fn try_load(&mut self) -> Result<(), HErr> {
        self.inner.replace(core::get()?);
        Ok(())
    }

    pub(crate) fn loaded(&self) -> bool {
        self.inner.is_some()
    }
}
