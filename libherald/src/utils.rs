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
                    "{error} at {file}:{line}:{column}, aborting",
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
                    "{error} at {file}:{line}:{column}, aborting",
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

#[cfg(tests)]
mod tests {
    #[test]
    fn strip_qrc() {
        let path = "file:///what/a/path".into();

        assert_eq!("/what/a/path", super::strip_qrc(path).unwrap());
    }
}
