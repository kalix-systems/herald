use herald_common::CapacityError;
use image;
use std::{fmt, sync::PoisonError};

#[derive(Debug)]
pub enum HErr {
    HeraldError(String),
    DatabaseError(rusqlite::Error),
    MutexError(String),
    InvalidUserId(String),
    IoError(std::io::Error),
    ImageError(String),
    SerializationError(String),
}

impl fmt::Display for HErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use HErr::*;
        match self {
            DatabaseError(e) => write!(f, "Database Error: {}", e),
            HeraldError(s) => write!(f, "Herald Error: {}", s),
            MutexError(s) => write!(f, "Mutex Error: {}", s),
            InvalidUserId(s) => write!(f, "InvalidUserId: {}", s),
            IoError(e) => write!(f, "IoError: {}", e),
            ImageError(s) => write!(f, "ImageError: {}", s),
            SerializationError(s) => write!(f, "SerializationError: {}", s),
        }
    }
}

impl std::error::Error for HErr {}

impl<T> From<PoisonError<T>> for HErr {
    fn from(e: PoisonError<T>) -> Self {
        HErr::MutexError(e.to_string())
    }
}

impl From<CapacityError<&str>> for HErr {
    fn from(e: CapacityError<&str>) -> Self {
        HErr::InvalidUserId(e.to_string())
    }
}

impl From<rusqlite::Error> for HErr {
    fn from(e: rusqlite::Error) -> Self {
        HErr::DatabaseError(e)
    }
}

impl From<std::io::Error> for HErr {
    fn from(e: std::io::Error) -> Self {
        HErr::IoError(e)
    }
}

impl From<image::ImageError> for HErr {
    fn from(e: image::ImageError) -> Self {
        use image::ImageError;
        match e {
            ImageError::IoError(e) => e.into(),
            e => HErr::ImageError(e.to_string()),
        }
    }
}

impl From<std::ffi::OsString> for HErr {
    fn from(e: std::ffi::OsString) -> Self {
        HErr::HeraldError(format!("Bad path: {:?}", e))
    }
}

impl From<serde_cbor::Error> for HErr {
    fn from(e: serde_cbor::Error) -> Self {
        HErr::SerializationError(e.to_string())
    }
}
