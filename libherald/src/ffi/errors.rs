use crate::errors::*;
use ffi_support::{ErrorCode, ExternError};

pub mod error_codes {
    use libc::c_int;

    /// Miscellaneous error from libherald.
    pub const HERALD_ERROR: c_int = 1;
    /// Error related to the database.
    pub const DATABASE_ERROR: c_int = 2;
    /// Tried to pass an invalid string as an argument.
    pub const INVALID_STRING: c_int = 3;
    /// Pass null pointer.
    pub const NULL_PTR: c_int = 4;
}

fn get_code(e: &HErr) -> ErrorCode {
    use HErr::*;
    match e {
        HeraldError(_) => ErrorCode::new(error_codes::HERALD_ERROR),
        DatabaseError(_) => ErrorCode::new(error_codes::DATABASE_ERROR),
        InvalidString => ErrorCode::new(error_codes::INVALID_STRING),
        NullPtr => ErrorCode::new(error_codes::NULL_PTR),
    }
}

impl From<HErr> for ExternError {
    fn from(e: HErr) -> Self {
        ExternError::new_error(get_code(&e), format!("{}", e))
    }
}
