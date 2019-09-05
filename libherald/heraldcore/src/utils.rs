use crate::errors::HErr;
use regex::{Regex, RegexBuilder};

/// Type alias for conversation id
pub type ConversationId = Vec<u8>;

const NUM_COLORS: u64 = 9;

pub(crate) fn id_to_color(id: &str) -> u32 {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    let mut state = DefaultHasher::default();
    id.hash(&mut state);
    (state.finish() % NUM_COLORS) as u32
}

/// Search strings
pub enum SearchPattern {
    /// Regex search string
    Regex(Regex),
    /// Normal search string
    Normal(Regex),
}

impl std::ops::Deref for SearchPattern {
    type Target = Regex;

    fn deref(&self) -> &Regex {
        match self {
            Self::Regex(re) => re,
            Self::Normal(re) => re,
        }
    }
}

impl SearchPattern {
    /// Creates a new `Regex` search pattern.
    pub fn new_regex(pattern: String) -> Result<Self, HErr> {
        Ok(Self::Regex(Regex::new(pattern.as_str())?))
    }

    /// Creates a new `Normal` search pattern.
    pub fn new_normal(pattern: String) -> Result<Self, HErr> {
        let pattern = format!(".*{}.*", pattern);

        Ok(Self::Normal(
            RegexBuilder::new(pattern.as_str())
                .dot_matches_new_line(true) // multiline match
                .case_insensitive(true) // case insensitivity
                .build()?,
        ))
    }
}
