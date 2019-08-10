use crate::errors::*;
use ffi_support::{ErrorCode, ExternError};

pub mod error_codes {
    use libc::c_int;

    pub const HERALD_ERROR: c_int = 1;
    pub const DATABASE_ERROR: c_int = 2;
    pub const MUTEX_ERROR: c_int = 3;
}

fn get_code(e: &HErr) -> ErrorCode {
    use HErr::*;
    match e {
        HeraldError(_) => ErrorCode::new(error_codes::HERALD_ERROR),
        DatabaseError(_) => ErrorCode::new(error_codes::DATABASE_ERROR),
        MutexError(_) => ErrorCode::new(error_codes::MUTEX_ERROR),
    }
}

impl From<HErr> for ExternError {
    fn from(e: HErr) -> Self {
        ExternError::new_error(get_code(&e), format!("{}", e))
    }
}
