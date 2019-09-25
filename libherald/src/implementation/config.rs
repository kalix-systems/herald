use crate::{interface::*, types::*};
use heraldcore::{abort_err, config::Config as Core};

pub struct Config {
    emit: ConfigEmitter,
    inner: Core,
}

impl ConfigTrait for Config {
    fn new(emit: ConfigEmitter) -> Self {
        let inner = abort_err!(Core::get());
        Config { emit, inner }
    }

    fn config_id(&self) -> FfiUserIdRef {
        self.inner.id.as_str()
    }

    fn name(&self) -> Option<&str> {
        self.inner.name.as_ref().map(|s| s.as_str())
    }

    fn set_name(&mut self, name: Option<String>) {
        match self.inner.set_name(name) {
            Ok(()) => self.emit.name_changed(),
            Err(e) => eprintln!("{}", e),
        }
        self.emit.display_name_changed();
    }

    /// Returns name if it is set, otherwise returns the user's id.
    fn display_name(&self) -> &str {
        match self.inner.name.as_ref() {
            Some(name) => name.as_str(),
            None => self.inner.id.as_str(),
        }
    }

    fn profile_picture(&self) -> Option<&str> {
        self.inner.profile_picture.as_ref().map(|s| s.as_str())
    }

    fn set_profile_picture(&mut self, picture: Option<String>) {
        match self.inner.set_profile_picture(picture) {
            Ok(()) => self.emit.profile_picture_changed(),
            Err(e) => eprintln!("{}", e),
        }
    }

    fn color(&self) -> u32 {
        self.inner.color
    }

    fn set_color(&mut self, color: u32) {
        match self.inner.set_color(color) {
            Ok(()) => self.emit.color_changed(),
            Err(e) => eprintln!("{}", e),
        }
    }

    fn colorscheme(&self) -> u32 {
        self.inner.colorscheme
    }

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
