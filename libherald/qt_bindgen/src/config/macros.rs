#[macro_export]
macro_rules! functions {
    (
        $(mut {
            $(
                $mut_key:ident ( $($mut_arg_name:ident : $mut_arg_type:expr),* ) => $mut_ret:expr
            ),*
        })?
        $(immutable {
            $(
                $key:ident ( $($arg_name:ident : $arg_type:expr),* ) => $ret:expr
            ),*
        })?
        $(no_magic {
            $($no_magic_key:ident : $value:expr),*
        })?
     )  => {
        {
            use rust_qt_binding_generator::configuration::SimpleType::*;
            let mut _map = ::std::collections::BTreeMap::new();
            $(
                $(
                let _key   = stringify!($mut_key).to_owned();
                let mut _value = Func::new($mut_ret).mutable();
                $(
                    let _value = _value.arg(stringify!($mut_arg_name), $mut_arg_type);
                )*
                let _ = _map.insert(stringify!($mut_key).to_owned(), _value.build());
                )*
            )?

            $(
                $(
                let _key   = stringify!($key).to_owned();
                let mut _value = Func::new($ret);
                $(
                    let _value = _value.arg(stringify!($arg_name), $arg_type);
                )*
                let _ = _map.insert(stringify!($key).to_owned(), _value.build());
                )*
            )?

            $(
                $(
                    let _ = _map.insert(stringify!($no_magic_key).to_owned(), $value.build());
                )*
            )?
            _map
        }
    };
}

#[macro_export]
macro_rules! functions_bak {
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
            #[allow(unused)]
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
