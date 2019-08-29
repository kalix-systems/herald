use serde::*;

#[macro_export]
macro_rules! serde_array {
    ($m: tt, $ty: ty, $u: expr) => {
        pub mod $m {
            use super::*;
            use arrayvec::*;
            use serde::*;
            use std::{mem, ptr};

            pub fn serialize<S: Serializer>(arr: &$ty, serializer: S) -> Result<S::Ok, S::Error> {
                arr.as_bytes().serialize(serializer)
            }

            pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<$ty, D::Error> {
                let slice: ArrayVec<[u8; $u]> = Deserialize::deserialize(de)?;
                <$ty>::from_bytes(&slice).map_err(|e| de::Error::custom(format!("{}", e)))
            }
        }
    };
}

#[macro_export]
macro_rules! deref_struct {
    ($ty: ty, $target: ty, $field: tt) => {
        impl std::ops::Deref for $ty {
            type Target = $target;
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }
    };
}

#[macro_export]
macro_rules! byte_array_hash {
    ($ty: ty, $field: tt) => {
        impl std::hash::Hash for $ty {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.$field.as_bytes().hash(state);
            }
        }
    };
}

#[macro_export]
macro_rules! byte_array_eq {
    ($ty: ty, $field: tt) => {
        impl PartialEq<$ty> for $ty {
            fn eq(&self, other: &Self) -> bool {
                self.$field.as_bytes() == other.$field.as_bytes()
            }
        }

        impl Eq for $ty {}
    };
}

#[macro_export]
macro_rules! byte_array_from {
    ($ty: tt, $inner: ty, $field: tt) => {
        impl<'a> TryFrom<&'a [u8]> for $ty {
            type Error = pqcrypto_traits::Error;

            fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
                <$inner>::from_bytes(value).map(|v| $ty { $field: v })
            }
        }
    };
}

#[macro_export]
macro_rules! byte_array_impls {
    ($ty: tt, $inner: ty, $field: tt) => {
        byte_array_hash!($ty, $field);
        byte_array_eq!($ty, $field);
        byte_array_from!($ty, $inner, $field);
    };
}

#[macro_export]
macro_rules! pub_secret_types {
    ($m: tt, $s_upper: expr, $p_upper: expr) => {
        serde_array!(asec, $m::SecretKey, $s_upper);
        serde_array!(apub, $m::PublicKey, $p_upper);

        #[derive(Serialize, Deserialize)]
        pub struct Sec {
            #[serde(with = "asec")]
            inner: $m::SecretKey,
        }

        #[derive(Serialize, Deserialize)]
        pub struct Pub {
            #[serde(with = "apub")]
            inner: $m::PublicKey,
        }

        byte_array_impls!(Pub, $m::PublicKey, inner);
        byte_array_impls!(Sec, $m::SecretKey, inner);
    };
}
