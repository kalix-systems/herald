#[derive(Debug, Clone, Copy)]
pub enum Error {
    UnknownStatus(i64),
    UnknownUserType(i64),
}

impl std::fmt::Display for Error {
    fn fmt(
        &self,
        out: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        use Error::*;
        match self {
            UnknownStatus(n) => write!(out, "Unknown user status: found {}, expected 0 or 1", n),
            UnknownUserType(n) => write!(out, "Unknown user type: found {}, expected 0 or 1", n),
        }
    }
}

impl std::error::Error for Error {}
