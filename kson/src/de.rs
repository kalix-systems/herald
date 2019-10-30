use super::{errors::*, *};
use bytes::Bytes;

pub struct Deserializer {
    pub data: Bytes,
    pub ix: usize,
}

pub trait De: Sized {
    fn de(from: &mut Deserializer) -> Result<Self, KsonError>;
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
            pub fn $fname(&mut self) -> Result<TagByte, KsonError> {
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

macro_rules! read_raw_uint {
    ($fname: ident, $type: tt, $len: expr) => {
        impl Deserializer {
            pub fn $fname(&mut self, len: u8) -> Result<$type, KsonError> {
                if len > $len {
                    e!(
                        IntTooShort {
                            tag_len: len,
                            max_len: $len
                        },
                        self.data.clone(),
                        self.ix
                    )
                } else if len as usize > self.remaining() {
                    e!(
                        LengthError {
                            expected: len as usize,
                            remaining: self.remaining(),
                        },
                        self.data.clone(),
                        self.ix
                    )
                }

                let bytes = &self.data[self.ix..self.ix + len as usize];
                self.ix += len as usize;

                let mut buf = [0u8; $len];
                unsafe {
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf.as_mut_ptr(), len as usize);
                }
                let out = $type::from_le_bytes(buf);

                Ok(out)
            }
        }
    };
}

read_raw_uint!(read_raw_u8, u8, 1);
read_raw_uint!(read_raw_u16, u16, 2);
read_raw_uint!(read_raw_u32, u32, 4);
read_raw_uint!(read_raw_u64, u64, 8);
read_raw_uint!(read_raw_u128, u128, 16);

macro_rules! read_uint_from_tag {
    ($fname: ident, $rawname: tt, $type: tt) => {
        impl Deserializer {
            pub fn $fname(&mut self, tag: TagByte) -> Result<$type, KsonError> {
                if !tag.is_big {
                    return Ok(tag.val as $type);
                }
                let len = tag.val + 1;
                self.$rawname(len)
            }
        }
    };
}

read_uint_from_tag!(read_u8_from_tag, read_raw_u8, u8);
read_uint_from_tag!(read_u16_from_tag, read_raw_u16, u16);
read_uint_from_tag!(read_u32_from_tag, read_raw_u32, u32);
read_uint_from_tag!(read_u64_from_tag, read_raw_u64, u64);
read_uint_from_tag!(read_u128_from_tag, read_raw_u128, u128);

impl Deserializer {
    pub fn read_raw_bytes(&mut self, len: usize) -> Result<Bytes, KsonError> {
        if self.remaining() < len {
            e!(
                LengthError {
                    expected: len,
                    remaining: self.remaining()
                },
                self.data.clone(),
                self.ix
            )
        }

        let out = self.data.slice(self.ix, self.ix + len);
        self.ix += len;

        Ok(out)
    }

    pub fn read_bytes_from_tag(&mut self, tag: TagByte) -> Result<Bytes, KsonError> {
        if tag.val & BYTES_ARE_UTF8 == BYTES_ARE_UTF8 {
            e!(
                WrongMinorType {
                    expected: "bytes",
                    found: "string"
                },
                self.data.clone(),
                self.ix
            )
        }

        let prelen = tag.val & !(MASK_TYPE | BIG_BIT | BYTES_ARE_UTF8);
        let len = {
            if !tag.is_big {
                prelen as usize
            } else {
                self.read_raw_u64(prelen)? as usize
            }
        };

        self.read_raw_bytes(len)
    }

    // TODO: replace this with string-y wrapper around bytes
    pub fn read_string_from_tag(&mut self, tag: TagByte) -> Result<String, KsonError> {
        if tag.val & BYTES_ARE_UTF8 != BYTES_ARE_UTF8 {
            e!(
                WrongMinorType {
                    expected: "string",
                    found: "bytes"
                },
                self.data.clone(),
                self.ix
            )
        }

        let prelen = tag.val & !(MASK_TYPE | BIG_BIT | BYTES_ARE_UTF8);
        let len = {
            if !tag.is_big {
                prelen as usize
            } else {
                self.read_raw_u64(prelen)? as usize
            }
        };

        let ix = self.ix;
        let bytes = self.read_raw_bytes(len)?;

        match std::str::from_utf8(&bytes) {
            Ok(s) => Ok(s.into()),
            Err(e) => Err(E!(BadUtf8String(e), self.data.clone(), ix)),
        }
    }
}
