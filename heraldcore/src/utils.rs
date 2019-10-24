use crate::errors::HErr;
use lazy_static::*;
use regex::{escape, Regex, RegexBuilder};
use serde::*;
use toml;

#[derive(Deserialize, Default)]
pub(crate) struct Conf {
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

        let conf = match toml::de::from_str(&file) {
            Ok(conf) => conf,
            Err(e) => {
                eprintln!("Error reading heraldcore configuration file: {}", e);
                return Self::default();
            }
        };
        conf
    }
}

lazy_static! {
    pub(crate) static ref CONF: Conf = Conf::read();
}

pub(crate) const RAND_ID_LEN: usize = 32;
const NUM_COLORS: u64 = 9;

pub(crate) fn id_to_color<H: std::hash::Hash>(id: H) -> u32 {
    use std::{collections::hash_map::DefaultHasher, hash::Hasher};

    let mut state = DefaultHasher::default();
    id.hash(&mut state);
    (state.finish() % NUM_COLORS) as u32
}

/// Search strings
pub enum SearchPattern {
    /// Regex search string
    Regex {
        /// Compiled regex
        pattern: Regex,
        /// Raw string
        raw: String,
    },
    /// Normal search string
    Normal {
        /// Compiled regex
        pattern: Regex,
        /// Raw string
        raw: String,
    },
}

impl std::ops::Deref for SearchPattern {
    type Target = Regex;

    fn deref(&self) -> &Regex {
        match self {
            Self::Regex { pattern, .. } => pattern,
            Self::Normal { pattern, .. } => pattern,
        }
    }
}

impl SearchPattern {
    /// Creates a new `Regex` search pattern.
    pub fn new_regex(raw: String) -> Result<Self, HErr> {
        Ok(Self::Regex {
            pattern: Regex::new(raw.as_str())?,
            raw,
        })
    }

    /// Creates a new `Normal` search pattern.
    pub fn new_normal(raw: String) -> Result<Self, HErr> {
        let pattern = escape(raw.as_str());

        Ok(Self::Normal {
            pattern: RegexBuilder::new(pattern.as_str())
                .dot_matches_new_line(true) // multiline match
                .case_insensitive(true) // case insensitivity
                .build()?,
            raw,
        })
    }

    /// Returns raw string of pattern
    pub fn raw(&self) -> &str {
        match self {
            Self::Regex { raw, .. } => raw.as_str(),
            Self::Normal { raw, .. } => raw.as_str(),
        }
    }

    /// Switches to regex mode
    pub fn regex_mode(&mut self) -> Result<(), HErr> {
        if let Self::Normal { raw, .. } = self {
            *self = Self::new_regex(raw.to_owned())?;
        }
        Ok(())
    }

    /// Switches to normal mode
    pub fn normal_mode(&mut self) -> Result<(), HErr> {
        if let Self::Regex { raw, .. } = self {
            *self = Self::new_normal(raw.to_owned())?;
        }
        Ok(())
    }

    /// Toggles regex mode
    pub fn toggle_mode(&mut self) -> Result<(), HErr> {
        match self {
            Self::Regex { raw, .. } => {
                *self = Self::new_normal(raw.to_owned())?;
            }
            Self::Normal { raw, .. } => {
                *self = Self::new_regex(raw.to_owned())?;
            }
        }

        Ok(())
    }
}

#[macro_export]
/// Convenience macro to abort on error.
macro_rules! abort_err {
    ($maybe: expr) => {
        match $maybe {
            Ok(val) => val,
            Err(e) => {
                eprintln!(
                    "{error} at {file}:{line}:{column}, aborting",
                    error = e,
                    file = file!(),
                    line = line!(),
                    column = column!()
                );
                std::process::abort();
            }
        }
    };
    ($maybe: expr, $msg: expr) => {
        match $maybe {
            Ok(val) => val,
            Err(e) => {
                eprintln!(
                    "{error} at {file}:{line}:{column}, message was {msg}, aborting",
                    error = e,
                    file = file!(),
                    line = line!(),
                    column = column!(),
                    msg = $msg
                );
                std::process::abort();
            }
        }
    };
}

#[macro_export]
/// Convenience macro for printing location of error.
macro_rules! womp {
    () => {
        &format!("{}:{}:{}", file!(), line!(), column!())
    };
    ($msg: expr) => {
        &format!("{} {}:{}:{}", $msg, file!(), line!(), column!())
    };
}

pub(crate) fn rand_id() -> [u8; RAND_ID_LEN] {
    use sodiumoxide::randombytes::randombytes_into;
    if sodiumoxide::init().is_err() {
        eprintln!("failed to init libsodium - what have you done");
        std::process::abort()
    }

    let mut buf = [0u8; RAND_ID_LEN];
    randombytes_into(&mut buf);
    buf
}
