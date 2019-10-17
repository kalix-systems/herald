/// Strips `qrc` prefix from paths passed from QML.
pub fn strip_qrc(mut path: String) -> String {
    path.split_off(7)
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
                let err_string = crate::utils::ret_err_string(&e, file!(), line!());

                eprintln!("{}", err_string);
                if crate::shared::errors::ERROR_QUEUE
                    .tx
                    .send(err_string)
                    .is_ok()
                {
                    crate::shared::errors::error_emit_try_poll();
                }
                return $retval;
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
                let err_string = crate::utils::ret_none_string(file!(), line!());

                eprintln!("{}", err_string);
                if crate::shared::errors::ERROR_QUEUE
                    .tx
                    .send(err_string)
                    .is_ok()
                {
                    crate::shared::errors::error_emit_try_poll();
                }
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
                crate::utils::bounds_chk_string($ix, $slf.list.len(), file!(), line!());

            eprint!("{}", err_string);

            if crate::shared::errors::ERROR_QUEUE
                .tx
                .send(err_string)
                .is_ok()
            {
                crate::shared::errors::error_emit_try_poll();
            }
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
