//! KSON is a simple serialization framework focused on combining strong performance with solid
//! ergonomics and a simple implementation.
//!
//! Things KSON does not try to do:
//! 1) Support multiple serialization backends - we may eventually support arbitrary `Write`
//!    backends, but for now we only support `Vec<u8>`
//! 2) Support deserializing from anything other than `Bytes`.

pub use location::loc;

pub mod prelude {
    pub use crate::{de::*, errors::*, ser::*, *};

    pub use arrayvec;
    pub use backtrace;
    pub use bytes::{self, Bytes};
}
pub mod de;
pub mod errors;
pub mod ser;
pub mod utils;
pub mod value;
pub use kson_derive::*;
use std::convert::TryFrom;

pub fn to_vec<T: ser::Ser + ?Sized>(t: &T) -> Vec<u8> {
    use ser::*;
    let mut out = Serializer::new();
    t.ser(&mut out);
    out.0
}

pub fn from_bytes<T: de::De>(from: prelude::Bytes) -> Result<T, errors::KsonError> {
    T::de(&mut de::Deserializer::new(from))
}

pub fn from_slice<T: de::De>(from: &[u8]) -> Result<T, errors::KsonError> {
    from_bytes(from.into())
}

pub const MASK_TYPE: u8 = 0b1110_0000;

pub const TYPE_OFFS: u8 = 5;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Type {
    Special = 0,
    Unsigned = 1,
    Signed = 2,
    Bytes = 3,
    Cons = 4,
    Collection = 5,
}

impl TryFrom<u8> for Type {
    type Error = u8;
    fn try_from(of: u8) -> Result<Type, u8> {
        match of {
            0 => Ok(Type::Special),
            1 => Ok(Type::Unsigned),
            2 => Ok(Type::Signed),
            3 => Ok(Type::Bytes),
            4 => Ok(Type::Cons),
            5 => Ok(Type::Collection),
            _ => Err(of),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Constants {
    False = 0,
    True = 1,
    Null = 2,
}

impl TryFrom<u8> for Constants {
    type Error = u8;
    fn try_from(of: u8) -> Result<Constants, u8> {
        match of {
            0 => Ok(Constants::False),
            1 => Ok(Constants::True),
            2 => Ok(Constants::Null),
            _ => Err(of),
        }
    }
}

impl From<bool> for Constants {
    fn from(of: bool) -> Constants {
        // does the compiler optimize this? can check later
        if of {
            Constants::True
        } else {
            Constants::False
        }
    }
}

pub const BIG_BIT: u8 = 0b0001_0000;

#[repr(u8)]
pub enum SignedType {
    I8 = 0,
    I16 = 1,
    I32 = 2,
    I64 = 3,
    I128 = 4,
}

impl SignedType {
    pub fn as_len(&self) -> usize {
        use SignedType::*;
        match self {
            I8 => 1,
            I16 => 2,
            I32 => 4,
            I64 => 8,
            I128 => 16,
        }
    }
}

impl std::convert::TryFrom<u8> for SignedType {
    type Error = u8;
    fn try_from(input: u8) -> Result<Self, u8> {
        use SignedType::*;
        match input {
            0 => Ok(I8),
            1 => Ok(I16),
            2 => Ok(I32),
            3 => Ok(I64),
            4 => Ok(I128),
            u => Err(u),
        }
    }
}

pub const BYTES_ARE_UTF8: u8 = 0b0000_1000;
pub const COLLECTION_IS_MAP: u8 = 0b0000_1000;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn types_fit() {
        let types = [
            Type::Special,
            Type::Unsigned,
            Type::Signed,
            Type::Bytes,
            Type::Cons,
            Type::Collection,
        ];

        for ty in &types {
            let ut = *ty as u8;
            assert_eq!(MASK_TYPE | (ut << TYPE_OFFS), MASK_TYPE);
            assert_eq!((ut << TYPE_OFFS) >> TYPE_OFFS, ut);
        }
    }
}
