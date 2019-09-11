use crate::errors::HErr;
use regex::{escape, Regex, RegexBuilder};

pub(crate) static DATE_FMT: &str = "%Y-%m-%d %H:%M:%S";
pub(crate) const RAND_ID_LEN: usize = 32;
const NUM_COLORS: u64 = 9;

pub(crate) fn rand_id() -> [u8; RAND_ID_LEN] {
    use rand::{thread_rng, RngCore};
    let mut rng = thread_rng();
    let mut buf = [0u8; RAND_ID_LEN];
    rng.fill_bytes(&mut buf);
    buf
}

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
                eprintln!("{}", e);
                std::process::abort();
            }
        }
    };
}
