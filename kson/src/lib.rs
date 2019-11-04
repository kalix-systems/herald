//! KSON is a simple serialization framework focused on combining strong performance with solid
//! ergonomics and a simple implementation.
//!
//! Things KSON does not try to do:
//! 1) Support multiple serialization backends - we may eventually support arbitrary `Write`
//!    backends, but for now we only support `Vec<u8>`
//! 2) Support deserializing from anything other than `Bytes`.

pub mod prelude {
    pub use crate::{de::*, errors::*, ser::*, *};

    pub use backtrace;
    pub use bytes::Bytes;
}
pub mod de;
pub mod errors;
pub mod ser;
pub mod utils;
pub mod value;
pub use kson_derive::*;

pub const MASK_TYPE: u8 = 0b1110_0000;

pub const TYPE_OFFS: u8 = 5;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Type {
    Special = 0 << TYPE_OFFS,
    Unsigned = 1 << TYPE_OFFS,
    Signed = 2 << TYPE_OFFS,
    Bytes = 3 << TYPE_OFFS,
    Cons = 4 << TYPE_OFFS,
    Collection = 5 << TYPE_OFFS,
}

pub const FALSE_BYTE: u8 = 0b0000_0000;
pub const TRUE_BYTE: u8 = 0b0000_0001;

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
            assert!(MASK_TYPE | *ty as u8 == MASK_TYPE);
        }
    }

    #[test]
    fn true_false_typed_corr() {
        assert_eq!(FALSE_BYTE & MASK_TYPE, Type::Special as u8);
        assert_eq!(TRUE_BYTE & MASK_TYPE, Type::Special as u8);
    }
}