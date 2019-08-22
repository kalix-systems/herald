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
        let (inner, init) = match Core::get() {
            Ok(c) => (c, true),
            Err(e) => {
                eprintln!("{}", e);
                let uninit_inner = Core {
                    id: None,
                    name: None,
                    profile_picture: None,
                };
                (uninit_inner, false)
            }
        };
        Config { emit, inner, init }
    }

    fn exists(&self) -> bool {
        self.init
    }

    fn emit(&mut self) -> &mut ConfigEmitter {
        &mut self.emit
    }

    fn set_id(&mut self, id: String) {
        if !self.init {
            self.inner = match Core::new(id, None, None) {
                Ok(c) => {
                    self.init = true;
                    c
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };
        }
    }

    fn id(&self) -> &str {
        match self.inner.id() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                ""
            }
        }
    }

    fn name(&self) -> Option<&str> {
        self.inner.name.as_ref().map(|s| s.as_str())
    }

    fn set_name(&mut self, name: Option<String>) {
        if let Err(e) = self.inner.set_name(name) {
            eprintln!("Error: {}", e);
        }
    }

    fn profile_picture(&self) -> Option<&str> {
        self.inner.profile_picture.as_ref().map(|p| p.as_str())
    }

    fn set_profile_picture(&mut self, picture: Option<String>) {
        if let Err(e) = self.inner.set_profile_picture(picture) {
            eprintln!("Error: {}", e);
        }
    }
}
