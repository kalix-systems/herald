use crate::interface::*;
use heraldcore::{config::Config as Core, db::DBTable};

pub struct Config {
    emit: ConfigEmitter,
    inner: Core,
    init: bool,
}

impl ConfigTrait for Config {
    fn new(emit: ConfigEmitter) -> Self {
        if let Err(e) = Core::create_table() {
            eprintln!("{}", e);
        }
        if let Err(e) = heraldcore::message::Messages::create_table() {
            eprintln!("{}", e);
        }
        if let Err(e) = heraldcore::contact::Contacts::create_table() {
            eprintln!("{}", e);
        }
        if let Err(e) = heraldcore::members::Members::create_table() {
            eprintln!("{}", e);
        }
        if let Err(e) = heraldcore::conversation::Conversations::create_table() {
            eprintln!("{}", e);
        }

        let (inner, init) = match Core::get() {
            Ok(c) => (c, true),
            Err(e) => {
                eprintln!("{}", e);
                let uninit_inner = Core {
                    id: None,
                    name: None,
                    profile_picture: None,
                    color: 0,
                    colorscheme: 0,
                };
                (uninit_inner, false)
            }
        };
        Config { emit, inner, init }
    }

    fn config_id(&self) -> &str {
        match self.inner.id() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                ""
            }
        }
    }

    fn set_config_id(&mut self, id: String) {
        if !self.init {
            self.inner = match Core::new(id, None, None, None, None) {
                Ok(c) => {
                    self.init = true;
                    self.emit.config_id_changed();
                    self.emit.init_changed();
                    c
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };
        }
    }

    fn name(&self) -> Option<&str> {
        self.inner.name.as_ref().map(|s| s.as_str())
    }

    fn set_name(&mut self, name: Option<String>) {
        match self.inner.set_name(name) {
            Ok(()) => self.emit.name_changed(),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    fn profile_picture(&self) -> Option<&str> {
        self.inner.profile_picture.as_ref().map(|p| p.as_str())
    }

    fn set_profile_picture(&mut self, picture: Option<String>) {
        match self
            .inner
            .set_profile_picture(crate::utils::strip_qrc(picture))
        {
            Ok(()) => self.emit.profile_picture_changed(),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    fn color(&self) -> u32 {
        self.inner.color
    }

    fn set_color(&mut self, color: u32) {
        match self.inner.set_color(color) {
            Ok(()) => self.emit.color_changed(),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    fn colorscheme(&self) -> u32 {
        self.inner.colorscheme
    }

    fn set_colorscheme(&mut self, colorscheme: u32) {
        match self.inner.set_colorscheme(colorscheme) {
            Ok(()) => self.emit.colorscheme_changed(),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    fn init(&self) -> bool {
        self.init
    }

    fn exists(&self) -> bool {
        self.init
    }

    fn emit(&mut self) -> &mut ConfigEmitter {
        &mut self.emit
    }
}
