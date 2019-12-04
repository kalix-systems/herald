pub use backtrace;
use std::fmt;

#[derive(Debug, Clone)]
/// A location in source code
pub struct Location {
    /// The line where the error occurred
    pub line: u32,
    /// The column where the error occurred
    pub col: u32,
    /// The file where the error occurred
    pub file: &'static str,
    /// The backtrace
    pub backtrace: backtrace::Backtrace,
}

impl fmt::Display for Location {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{file}:{line}:{column}\n{backtrace:#?}",
            file = self.file,
            line = self.line,
            column = self.file,
            backtrace = self.backtrace
        )
    }
}

#[macro_export]
/// Returns the location this macro was called from
macro_rules! loc {
    () => {
        $crate::Location {
            file: file!(),
            line: line!(),
            col: column!(),
            backtrace: backtrace::Backtrace::new(),
        }
    };
}
