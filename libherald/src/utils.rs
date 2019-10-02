/// Strips `qrc` prefix from paths passed from QML.
pub fn strip_qrc(path: Option<String>) -> Option<String> {
    let mut path = path;

    match &mut path {
        Some(path) => {
            let stripped = path.split_off(7);
            Some(stripped)
        }
        None => None,
    }
}

#[macro_export]
/// Early return on error
macro_rules! ret_err {
    ($maybe: expr) => {
        match $maybe {
            Ok(val) => val,
            Err(e) => {
                eprintln!(
                    "{error} at {file}:{line}:{column}",
                    error = e,
                    file = file!(),
                    line = line!(),
                    column = column!()
                );
                return;
            }
        }
    };
    ($maybe: expr, $retval: expr) => {
        match $maybe {
            Ok(val) => val,
            Err(e) => {
                eprintln!(
                    "{error} at {file}:{line}:{column}",
                    error = e,
                    file = file!(),
                    line = line!(),
                    column = column!()
                );
                return $retval;
            }
        }
    };
}

#[macro_export]
/// Early return on unexpected `None`
macro_rules! ret_none {
    ($maybe: expr) => {
        match $maybe {
            Some(val) => val,
            None => {
                eprintln!(
                    "Unexpected `None` at {file}:{line}:{column}",
                    file = file!(),
                    line = line!(),
                    column = column!()
                );
                return;
            }
        }
    };
    ($maybe: expr, $retval: expr) => {
        match $maybe {
            Some(val) => val,
            None => {
                eprintln!(
                    "Unexpected `None` at {file}:{line}:{column}",
                    file = file!(),
                    line = line!(),
                    column = column!()
                );
                return $retval;
            }
        }
    };
}

#[macro_export]
/// Performs a bounds check
macro_rules! bounds_chk {
    ($slf: expr, $ix: expr) => {
        if $slf.list.len().saturating_sub(1) < $ix {
            return;
        }
    };
    ($slf: expr, $ix: expr, $retval: expr) => {
        if $slf.list.len().saturating_sub(1) < $ix {
            return $retval;
        }
    };
}

#[cfg(tests)]
mod tests {
    #[test]
    fn strip_qrc() {
        let path = "file:///what/a/path".into();

        assert_eq!("/what/a/path", super::strip_qrc(path).unwrap());
    }
}
