use lazy_static::*;
use serde::*;
use toml;

#[derive(Deserialize, Default)]
pub struct Conf {
    /// Server address, e.g., `127.0.0.1:8080`
    pub server_addr: Option<String>,
}

impl Conf {
    #[cfg(not(any(android, ios)))]
    fn config_path() -> Option<String> {
        // happens at runtime
        std::env::var("HERALDCORE_CONF").ok()
    }

    #[cfg(any(android, ios))]
    fn config_path() -> Option<String> {
        // happens at compile time
        option_env!("HERALDCORE_CONF")
    }

    pub(crate) fn read() -> Self {
        let path = match Self::config_path() {
            Some(path) => path,
            None => return Self::default(),
        };

        let file = match std::fs::read_to_string(path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error reading heraldcore configuration file: {}", e);
                return Self::default();
            }
        };

        match toml::de::from_str(&file) {
            Ok(conf) => conf,
            Err(e) => {
                eprintln!("Error reading heraldcore configuration file: {}", e);
                Self::default()
            }
        }
    }
}

lazy_static! {
    pub static ref CONF: Conf = Conf::read();
}
