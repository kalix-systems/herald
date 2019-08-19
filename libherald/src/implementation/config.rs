use crate::interface::*;
use heraldcore::config::Config as Core;

pub struct Config {
    emit: ConfigEmitter,
    id: String,
    name: Option<String>,
    profile_picture: Option<Vec<u8>>,
}

impl ConfigTrait for Config {
    fn new(emit: ConfigEmitter) -> Self {
        Config {
            emit,
            id: "".into(),
            name: None,
            profile_picture: None,
        }
    }

    fn emit(&mut self) -> &mut ConfigEmitter {
        &mut self.emit
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }

    fn set_name(&mut self, name: Option<String>) {
        self.name = name;
        let name = self.name.as_ref().map(|s| s.as_str());

        if let Err(e) = Core::update_name(name) {
            eprintln!("Error: {}", e);
        }
    }

    fn profile_picture(&self) -> Option<&[u8]> {
        self.profile_picture.as_ref().map(|p| p.as_slice())
    }

    fn set_profile_picture(&mut self, picture: Option<&[u8]>) {
        self.profile_picture = picture.map(|p| p.to_vec());
        if let Err(e) = Core::update_profile_picture(picture) {
            eprintln!("Error: {}", e);
        }
    }
}
