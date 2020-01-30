#[macro_export]
macro_rules! sql {
    ($path: literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/sql/",
            $path,
            ".sql"
        ))
    };
}

#[macro_export]
macro_rules! types {
    ($($typ: ident,)+) => (types!($($typ),+));

    ( $($typ:ident),* ) => {
        &[$(Type::$typ, )*]
    }
}

#[macro_export]
macro_rules! params {
    ($($val:expr,)+) => (params!($($val),+));

    ( $($val:expr),* ) => {
        &[$(&$val, )*]
    }
}
