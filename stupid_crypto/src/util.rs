use serde::*;

#[macro_export]
macro_rules! serde_array {
    ($m:ident, $n:expr, $u: expr) => {
        pub mod $m {
            use super::*;
            use arrayvec::*;
            use serde::*;
            use std::{mem, ptr};

            pub fn serialize<S: Serializer>(
                arr: &[u8; $n],
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                (arr as &[u8]).serialize(serializer)
            }

            pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<[u8; $n], D::Error> {
                let slice: ArrayVec<[u8; $u]> = Deserialize::deserialize(de)?;
                if slice.len() != $n {
                    Err(de::Error::custom("input slice has wrong length"))
                } else {
                    let mut result = [0u8; $n];
                    for (src, dst) in slice.into_iter().zip(&mut result[..]) {
                        *dst = src;
                    }
                    Ok(result)
                }
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
                (&self.$field as &[u8]).hash(state);
            }
        }
    };
}

#[macro_export]
macro_rules! byte_array_eq {
    ($ty: ty, $field: tt) => {
        impl PartialEq<$ty> for $ty {
            fn eq(&self, other: &Self) -> bool {
                (&self.$field as &[u8]) == (&other.$field as &[u8])
            }
        }

        impl Eq for $ty {}
    };
}

#[macro_export]
macro_rules! byte_array_from {
    ($ty: tt, $len: expr) => {
        impl<'a> TryFrom<&'a [u8]> for $ty {
            type Error = ();

            fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
                if value.len() == $len {
                    let mut inner = [0u8; $len];
                    inner.copy_from_slice(value);
                    Ok($ty { inner })
                } else {
                    Err(())
                }
            }
        }
    };
}
