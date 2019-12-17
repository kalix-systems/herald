use backtrace::Backtrace;
use bytes::Bytes;
use location::Location;
use std::{fmt, str::Utf8Error};

pub type KsonError = Box<KsonErrorInner>;

#[derive(Debug, Clone)]
pub struct KsonErrorInner {
    pub backtrace: Backtrace,
    pub location: Location,
    pub message: Option<String>,
    pub bytes: Bytes,
    pub offset: usize,
    pub variant: Variant,
}

#[derive(Debug, Clone)]
pub enum Variant {
    LengthError {
        expected: usize,
        remaining: usize,
    },
    WrongType {
        expected: crate::Type,
        found: u8,
    },
    IntTooShort {
        tag_len: u8,
        max_len: u8,
    },
    WrongMinorType {
        expected: &'static str,
        found: String,
    },
    WrongEnumVariant {
        found: String,
    },
    WrongConsSize {
        expected: usize,
        found: usize,
    },
    WrongConsKey {
        expected: &'static str,
        found: String,
    },
    CollectionTooLarge {
        max_len: usize,
        found: usize,
    },
    WrongCollSize {
        expected: usize,
        found: usize,
    },
    BadUtf8String(Utf8Error),
    UnknownConst(u8),
    UnknownType(u8),
    CustomError(String),
}

use Variant::*;

impl fmt::Display for KsonErrorInner {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "Error while deserializing at {}", self.location)?;
        if let Some(msg) = &self.message {
            write!(f, " with extra msg {}", msg)?;
        }
        write!(
            f,
            "\n\
             Raw bytes were: {:#?}\n\
             Error found at offset: {}\n\
             Variant was: {}\n\
             {:#?}
             ",
            self.bytes,
            self.offset,
            (match &self.variant {
                LengthError {
                    expected,
                    remaining,
                } => format!(
                    "expected {} bytes but only {} remained",
                    expected, remaining
                ),
                WrongType { expected, found } => {
                    format!("expected type {:?} but only found {:x?}", expected, found)
                }
                IntTooShort { tag_len, max_len } => format!(
                    "tried to deserialize int of length {} as type with max len {}",
                    tag_len, max_len
                ),
                WrongMinorType { expected, found } => {
                    format!("expected minor type {} but found {}", expected, found)
                }
                WrongConsSize { expected, found } => format!(
                    "cons had wrong size - expected {} fields but only found {}",
                    expected, found
                ),
                WrongEnumVariant { found } => {
                    format!("tried to parse nonexistent enum variant {}", found)
                }
                WrongConsKey { expected, found } => format!(
                    "cons had wrong keys - expected {}, found {}",
                    expected, found
                ),
                CollectionTooLarge { max_len, found } => format!(
                    "collection was too large - had capacity for {} elements but {} were found",
                    max_len, found
                ),
                WrongCollSize { expected, found } => format!(
                    "collection was wrong size - expected {} elements but stated length was {}",
                    expected, found
                ),
                BadUtf8String(u) => format!("bad utf-8 string, error was {}", u),
                UnknownType(u) => format!("unknown type found: {}", u),
                UnknownConst(u) => format!("unknown constant with value {:x?}", u),
                CustomError(s) => s.clone(),
            }),
            self.backtrace
        )
    }
}

impl std::error::Error for KsonErrorInner {}

#[macro_export]
macro_rules! E {
    ($var: expr, $byt: expr, $offset: expr, $($t: tt),*) => {
        Box::new($crate::errors::KsonErrorInner {
            backtrace: $crate::prelude::backtrace::Backtrace::new(),
            location: $crate::loc!(),
            bytes: $byt,
            offset: $offset,
            variant: {
                use $crate::errors::Variant::*;
                $var
            },
            message: Some(format!($($t),*))
        })
    };

    ($var: expr, $byt: expr, $offset: expr) => {
        Box::new($crate::errors::KsonErrorInner {
            backtrace: $crate::prelude::backtrace::Backtrace::new(),
            location: $crate::loc!(),
            bytes: $byt,
            offset: $offset,
            variant: {
                use $crate::errors::Variant::*;
                $var
            },
            message: None,
        })
    };
}

#[macro_export]
macro_rules! e {
    ($var: expr, $byt: expr, $offset: expr, $($t:tt),*) => {
        {
            return Err(E!($var, $byt, $offset, $($t),*));
        }
    };

    ($var: expr, $byt: expr, $offset: expr) => {
        {
            return Err(E!($var, $byt, $offset));
        }
    };
}
