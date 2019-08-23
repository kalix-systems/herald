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

#[cfg(tests)]
mod tests {
    #[test]
    fn strip_qrc() {
        let path = "file:///what/a/path".into();

        assert_eq!("/what/a/path", super::strip_qrc(path).unwrap());
    }
}
