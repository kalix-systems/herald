#[macro_export]
/// Convenience macro to exit on error.
macro_rules! exit_err {
    ($maybe: expr) => {
        match $maybe {
            Ok(val) => val,
            Err(e) => {
                use ::std::io::Write;
                let mut se = ::std::io::stderr();

                writeln!(
                    &mut se,
                    "{error} at {file}:{line}:{column}, exiting",
                    error = e,
                    file = file!(),
                    line = line!(),
                    column = column!()
                )
                .ok();

                // TODO error codes
                ::std::process::exit(1);
            }
        }
    };
    ($maybe: expr, $msg: expr) => {
        match $maybe {
            Ok(val) => val,
            Err(e) => {
                use ::std::io::Write;
                let mut se = ::std::io::stderr();

                writeln!(
                    &mut se,
                    "{error} at {file}:{line}:{column}, message was {msg}, exiting",
                    error = e,
                    file = file!(),
                    line = line!(),
                    column = column!(),
                    msg = $msg
                )
                .ok();

                ::std::process::exit(1);
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

#[macro_export]
/// Convenience macro
macro_rules! w {
    ($maybe: expr) => {{
        match $maybe {
            Ok(val) => val,
            Err(e) => {
                use $crate::womp;
                eprintln!("{}", womp!());
                return Err(e.into());
            }
        }
    }};
}
