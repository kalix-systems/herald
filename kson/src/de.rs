use super::{errors::*, utils::*, *};
use bytes::Bytes;

pub struct Deserializer {
    pub data: Bytes,
    pub ix: usize,
}

pub trait De: Sized {
    fn de(from: &mut Deserializer) -> Result<Self, Error>;
}

pub struct TagByte {
    pub is_big: bool,
    pub val: u8,
}

impl Deserializer {
    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.ix)
    }
}

macro_rules! tag_reader_method {
    ($fname: ident, $type: tt, $message: expr) => {
        impl Deserializer {
            pub fn $fname(&mut self) -> Result<TagByte, Error> {
                if self.remaining() == 0 {
                    e!(
                        LengthError {
                            expected: 1,
                            remaining: 0
                        },
                        self.data.clone(),
                        self.ix,
                        $message
                    )
                }

                let byte = self.data[self.ix];

                if byte & MASK_TYPE != $crate::Type::$type as u8 {
                    e!(
                        WrongType {
                            expected: $crate::Type::$type,
                            found: byte & $crate::MASK_TYPE,
                        },
                        self.data.clone(),
                        self.ix,
                        $message
                    )
                }

                self.ix += 1;

                Ok(TagByte {
                    is_big: byte & BIG_BIT == BIG_BIT,
                    val: byte & !(MASK_TYPE | BIG_BIT),
                })
            }
        }
    };
}

tag_reader_method!(read_uint_tag, Unsigned, "failed to read uint tag");
tag_reader_method!(read_int_tag, Signed, "failed to read int tag");
tag_reader_method!(read_bytes_tag, Bytes, "failed to read byte tag");

macro_rules! read_uint_from_tag {
    ($fname: ident, $type: tt, $len: expr) => {
        impl Deserializer {
            pub fn $fname(&mut self, tag: TagByte) -> Result<$type, Error> {
                if !tag.is_big {
                    return Ok(tag.val as $type);
                } else if tag.val > $len {
                    e!(
                        IntTooShort {
                            tag_len: tag.val,
                            max_len: $len
                        },
                        self.data.clone(),
                        self.ix,
                        "failed to read uint after tag"
                    )
                } else if tag.val as usize > self.remaining() {
                    e!(
                        LengthError {
                            expected: tag.val as usize,
                            remaining: self.remaining(),
                        },
                        self.data.clone(),
                        self.ix,
                        "failed to read uint after tag"
                    )
                }

                let bytes = &self.data[self.ix..self.ix + tag.val as usize];
                self.ix += tag.val as usize;

                let mut buf = [0u8; $len];
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        bytes.as_ptr(),
                        buf.as_mut_ptr(),
                        tag.val as usize,
                    );
                }
                let out = $type::from_le_bytes(buf);

                Ok(out)
            }
        }
    };
}

read_uint_from_tag!(read_u8_from_tag, u8, 1);
read_uint_from_tag!(read_u16_from_tag, u16, 2);
read_uint_from_tag!(read_u32_from_tag, u32, 4);
read_uint_from_tag!(read_u64_from_tag, u64, 8);
read_uint_from_tag!(read_u128_from_tag, u128, 16);
