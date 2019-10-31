/// Strips `qrc` prefix from paths passed from QML.
pub fn strip_qrc(mut path: String) -> String {
    path.split_off(7)
}

pub(crate) fn err_string_msg(
    e: &dyn std::error::Error,
    file: &str,
    line: u32,
    msg: &'static str,
) -> String {
    format!(
        "{msg}: {error} at {file}:{line}",
        msg = msg,
        error = e,
        file = file,
        line = line,
    )
}

pub(crate) fn ret_err_string(e: &dyn std::error::Error, file: &str, line: u32) -> String {
    format!(
        "{error} at {file}:{line}",
        error = e,
        file = file,
        line = line,
    )
}

#[macro_export]
/// Early return on error
macro_rules! ret_err {
    ($maybe: expr) => {
        ret_err!($maybe, ())
    };
    ($maybe: expr, $retval: expr) => {
        match $maybe {
            Ok(val) => val,
            Err(e) => {
                use $crate::shared::SingletonBus;
                let err_string = crate::utils::ret_err_string(&e, file!(), line!());

                eprintln!("{}", err_string);
                $crate::imp::errors::Errors::push(err_string).ok();
                return $retval;
            }
        }
    };
}

#[macro_export]
/// If the value passed is an error, ushes an errors to the error queue without an early return.
macro_rules! push_err {
    ($maybe: expr, $msg: expr) => {
        match $maybe {
            Ok(val) => val,
            Err(e) => {
                use $crate::shared::SingletonBus;
                let err_string = crate::utils::err_string_msg(&e, file!(), line!(), $msg);

                eprintln!("{}", err_string);
                $crate::imp::errors::Errors::push(err_string).ok();
            }
        }
    };
}

pub(crate) fn ret_none_string(file: &str, line: u32) -> String {
    format!(
        "Unexpected `None` at {file}:{line}",
        file = file,
        line = line,
    )
}

#[macro_export]
/// Early return on unexpected `None`
macro_rules! ret_none {
    ($maybe: expr) => {
        ret_none!($maybe, ())
    };
    ($maybe: expr, $retval: expr) => {
        match $maybe {
            Some(val) => val,
            None => {
                use $crate::shared::SingletonBus;
                let err_string = $crate::utils::ret_none_string(file!(), line!());

                eprintln!("{}", err_string);
                $crate::imp::errors::Errors::push(err_string).ok();
                return $retval;
            }
        }
    };
}

pub(crate) fn bounds_chk_string(ix: usize, len: usize, file: &str, line: u32) -> String {
    format!(
        "Tried get index {ix} from a list of length {actual} at {file}:{line}",
        ix = ix,
        actual = len,
        file = file,
        line = line,
    )
}

#[macro_export]
/// Performs a bounds check
macro_rules! bounds_chk {
    ($slf: expr, $ix: expr) => {
        bounds_chk!($slf, $ix, ())
    };
    ($slf: expr, $ix: expr, $retval: expr) => {
        if $slf.list.len().saturating_sub(1) < $ix {
            let err_string =
                $crate::utils::bounds_chk_string($ix, $slf.list.len(), file!(), line!());

            eprint!("{}", err_string);

            $crate::imp::errors::Errors::push(err_string).ok();
            return $retval;
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn strip_qrc() {
        let path = "file:///what/a/path".into();

        assert_eq!("/what/a/path", super::strip_qrc(path));
    }
}
