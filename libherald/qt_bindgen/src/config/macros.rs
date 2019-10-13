#[macro_export]
macro_rules! functions {
    ( $($key:ident : $value:expr),* ) => {
        {
            use rust_qt_binding_generator::configuration::SimpleType::*;
            let mut _map = ::std::collections::BTreeMap::new();
            $(
                let _ = _map.insert(stringify!($key).to_owned(), $value.build());
            )*
            _map
        }
    };
}

#[macro_export]
macro_rules! objects {
    ( $($value:expr),* ) => {
        {
            use std::rc::Rc;
            let mut _map = ::std::collections::BTreeMap::new();
            $(
                let _val = $value;
                let _ = _map.insert(_val.name.clone(), Rc::new(_val));
            )*
            _map
        }
    };
}

#[macro_export]
macro_rules! props {
    ( $($key:ident : $value:expr),* ) => {
        {
            use rust_qt_binding_generator::configuration::SimpleType::*;
            let mut _map = ::std::collections::BTreeMap::new();
            $(
                let _ = _map.insert(stringify!($key).to_owned(), $value.build());
            )*
            _map
        }
    };
}

#[macro_export]
macro_rules! item_props {
    ( $($key:ident : $value:expr),* ) => {
        {
            use rust_qt_binding_generator::configuration::SimpleType::*;
            let mut _map = ::std::collections::BTreeMap::new();
            $(
                let _ = _map.insert(stringify!($key).to_owned(), $value.build());
            )*
            _map
        }
    };
}

#[macro_export]
macro_rules! obj {
    ($name:ident : $value:expr) => {
        $value.name(stringify!($name)).build().unwrap()
    };
}
