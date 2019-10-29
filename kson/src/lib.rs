//! KSON is a simple serialization framework focused on combining strong performance with solid
//! ergonomics and a simple implementation.
//!
//! Things KSON does not try to do:
//! 1) Support multiple serialization backends - we may eventually support arbitrary `Write`
//!    backends, but for now we only support `Vec<u8>`
//! 2) Support deserializing from anything other than `Bytes`.

pub mod de;
pub mod errors;
pub mod ser;
pub mod utils;
pub mod value;

pub const MASK_TYPE: u8 = 0b1110_0000;
pub const TYPE_OFFS: u8 = 5;

#[repr(u8)]
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

pub const BYTES_ARE_UTF8: u8 = 0b0000_1000;

pub const COLLECTION_IS_MAP: u8 = 0b0000_1000;
