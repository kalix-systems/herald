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
                ::std::process::abort();
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
                ::std::process::abort();
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

#[macro_export]
/// Trivial conversion helper macro.
macro_rules! from_fn {
    ($to:ty, $from:ty, $fn:expr) => {
        impl From<$from> for $to {
            fn from(f: $from) -> $to {
                $fn(f)
            }
        }
    };
}
