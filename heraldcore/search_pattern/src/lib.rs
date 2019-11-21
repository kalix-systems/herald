pub use regex::Error as SearchPatternError;
use regex::{escape, Error, Regex, RegexBuilder};

/// Search strings
#[derive(Clone, Debug)]
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
    pub fn new_regex(raw: String) -> Result<Self, Error> {
        Ok(Self::Regex {
            pattern: Regex::new(raw.as_str())?,
            raw,
        })
    }

    /// Creates a new `Normal` search pattern.
    pub fn new_normal(raw: String) -> Result<Self, Error> {
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

    /// Indicates whether the `SearchPattern` is a regex pattern.
    pub fn is_regex(&self) -> bool {
        match self {
            SearchPattern::Normal { .. } => false,
            SearchPattern::Regex { .. } => true,
        }
    }

    /// Indicates whether the `SearchPattern` is a regex pattern.
    pub fn is_normal(&self) -> bool {
        match self {
            SearchPattern::Normal { .. } => true,
            SearchPattern::Regex { .. } => false,
        }
    }

    /// Switches to regex mode
    pub fn regex_mode(&mut self) -> Result<(), Error> {
        if let Self::Normal { raw, .. } = self {
            *self = Self::new_regex(raw.to_owned())?;
        }
        Ok(())
    }

    /// Switches to normal mode
    pub fn normal_mode(&mut self) -> Result<(), Error> {
        if let Self::Regex { raw, .. } = self {
            *self = Self::new_normal(raw.to_owned())?;
        }
        Ok(())
    }

    /// Toggles regex mode
    pub fn toggle_mode(&mut self) -> Result<(), Error> {
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

    /// Changes the pattern while preserving the mode
    pub fn set_pattern(
        &mut self,
        pattern: String,
    ) -> Result<(), Error> {
        match self {
            Self::Regex { .. } => {
                *self = Self::new_regex(pattern)?;
            }
            Self::Normal { .. } => {
                *self = Self::new_normal(pattern)?;
            }
        }

        Ok(())
    }
}
