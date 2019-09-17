use crate::interface::*;
use herald_common::UserIdRef;
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

    fn config_id(&self) -> UserIdRef {
        self.inner.id()
    }

    fn name(&self) -> Option<&str> {
        self.inner.name.as_ref().map(|s| s.as_str())
    }

    fn set_name(&mut self, name: Option<String>) {
        match self.inner.set_name(name) {
            Ok(()) => self.emit.name_changed(),
            Err(e) => eprintln!("{}", e),
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
        match self.inner.set_color(colorscheme) {
            Ok(()) => self.emit.colorscheme_changed(),
            Err(e) => eprintln!("{}", e),
        }
    }

    fn emit(&mut self) -> &mut ConfigEmitter {
        &mut self.emit
    }
}
