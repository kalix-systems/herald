use crate::interface::*;
use heraldcore::{
    config::{Config as Core, ConfigBuilder},
    db::DBTable,
};

pub struct Config {
    emit: ConfigEmitter,
    inner: Option<Core>,
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

        let inner = match Core::get() {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        };
        Config { emit, inner }
    }

    fn config_id(&self) -> &str {
        match &self.inner {
            Some(s) => s.id(),
            None => {
                eprintln!("Config id not set");
                ""
            }
        }
    }

    fn set_config_id(&mut self, id: String) {
        if self.inner.is_none() {
            self.inner = match ConfigBuilder::new(id).build() {
                Ok(c) => {
                    self.emit.config_id_changed();
                    self.emit.init_changed();
                    Some(c)
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };
        }
    }

    fn name(&self) -> Option<&str> {
        match &self.inner {
            Some(inner) => inner.name.as_ref().map(|s| s.as_str()),
            None => None,
        }
    }

    fn set_name(&mut self, name: Option<String>) {
        match &mut self.inner {
            Some(inner) => match inner.set_name(name) {
                Ok(()) => self.emit.name_changed(),
                Err(e) => eprintln!("{}", e),
            },
            None => {
                eprintln!("Config id not set");
            }
        }
    }

    fn profile_picture(&self) -> Option<&str> {
        match &self.inner {
            Some(inner) => inner.profile_picture.as_ref().map(|s| s.as_str()),
            None => None,
        }
    }

    fn set_profile_picture(&mut self, picture: Option<String>) {
        match &mut self.inner {
            Some(inner) => match inner.set_profile_picture(picture) {
                Ok(()) => self.emit.profile_picture_changed(),
                Err(e) => eprintln!("{}", e),
            },
            None => {
                eprintln!("Config id not set");
            }
        }
    }

    fn color(&self) -> u32 {
        match &self.inner {
            Some(inner) => inner.color,
            None => 0,
        }
    }

    fn set_color(&mut self, color: u32) {
        match &mut self.inner {
            Some(inner) => match inner.set_color(color) {
                Ok(()) => self.emit.color_changed(),
                Err(e) => eprintln!("{}", e),
            },
            None => {
                eprintln!("Config id not set");
            }
        }
    }

    fn colorscheme(&self) -> u32 {
        match &self.inner {
            Some(inner) => inner.colorscheme,
            None => 0,
        }
    }

    fn set_colorscheme(&mut self, colorscheme: u32) {
        match &mut self.inner {
            Some(inner) => match inner.set_color(colorscheme) {
                Ok(()) => self.emit.color_changed(),
                Err(e) => eprintln!("{}", e),
            },
            None => {
                eprintln!("Config id not set");
            }
        }
    }
    fn init(&self) -> bool {
        self.inner.is_some()
    }

    fn exists(&self) -> bool {
        self.inner.is_some()
    }

    fn emit(&mut self) -> &mut ConfigEmitter {
        &mut self.emit
    }
}
