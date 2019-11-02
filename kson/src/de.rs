use super::{errors::*, *};
use bytes::Bytes;
use std::convert::TryFrom;

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
    pub fn new(source: Bytes) -> Self {
        Deserializer {
            data: source,
            ix: 0,
        }
    }
    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.ix)
    }

    pub fn read_raw_slice(&mut self, len: usize) -> Result<&[u8], KsonError> {
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
        let out = &self.data[self.ix..self.ix + len];
        self.ix += len;
        Ok(out)
    }

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
tag_reader_method!(read_coll_tag, Collection, "failed to read collection tag");
tag_reader_method!(read_cons_tag, Cons, "failed to read cons tag");

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
                }

                let bytes = self.read_raw_slice(len as usize)?;

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
                self.$rawname(tag.val + 1)
            }
        }
    };
}

read_uint_from_tag!(read_u8_from_tag, read_raw_u8, u8);
read_uint_from_tag!(read_u16_from_tag, read_raw_u16, u16);
read_uint_from_tag!(read_u32_from_tag, read_raw_u32, u32);
read_uint_from_tag!(read_u64_from_tag, read_raw_u64, u64);
read_uint_from_tag!(read_u128_from_tag, read_raw_u128, u128);

macro_rules! read_int_from_tag {
    ($fname: ident, $type: tt, $len: expr) => {
        impl Deserializer {
            pub fn $fname(&mut self, tag: TagByte) -> Result<$type, KsonError> {
                if !tag.is_big {
                    return Ok(tag.val as $type);
                }

                let len = SignedType::try_from(tag.val)
                    .map_err(|u| {
                        E!(
                            WrongMinorType {
                                expected: "signed integer",
                                found: "unknown".into(),
                            },
                            self.data.clone(),
                            self.ix,
                            "tried to read signed integer,\
                             but tag value was not a signed integer type {}",
                            u
                        )
                    })?
                    .as_len();

                if len > $len {
                    e!(
                        IntTooShort {
                            tag_len: len as u8,
                            max_len: $len
                        },
                        self.data.clone(),
                        self.ix
                    )
                }

                let bytes = self.read_raw_slice(len)?;

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

read_int_from_tag!(read_i8_from_tag, i8, 1);
read_int_from_tag!(read_i16_from_tag, i16, 2);
read_int_from_tag!(read_i32_from_tag, i32, 4);
read_int_from_tag!(read_i64_from_tag, i64, 8);
read_int_from_tag!(read_i128_from_tag, i128, 16);

impl Deserializer {
    pub fn read_bool(&mut self) -> Result<bool, KsonError> {
        let tag = self.read_raw_u8(1)?;
        if tag & MASK_TYPE == Type::Special as u8 {
            match tag & !MASK_TYPE {
                FALSE_BYTE => Ok(false),
                TRUE_BYTE => Ok(true),
                other => Err(E!(UnknownConst(other), self.data.clone(), self.ix)),
            }
        } else {
            Err(E!(
                WrongType {
                    expected: Type::Special,
                    found: tag & MASK_TYPE
                },
                self.data.clone(),
                self.ix
            ))
        }
    }
    pub fn read_bytes_from_tag(&mut self, tag: TagByte) -> Result<Bytes, KsonError> {
        if tag.val & BYTES_ARE_UTF8 == BYTES_ARE_UTF8 {
            e!(
                WrongMinorType {
                    expected: "bytes",
                    found: "string".into()
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
    pub fn read_str_from_tag(&mut self, tag: TagByte) -> Result<&str, KsonError> {
        if tag.val & BYTES_ARE_UTF8 != BYTES_ARE_UTF8 {
            e!(
                WrongMinorType {
                    expected: "string",
                    found: "bytes".into()
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
                self.read_raw_u64(prelen + 1)? as usize
            }
        };

        let ix = self.ix;
        let err_data = self.data.clone();
        let bytes = self.read_raw_slice(len)?;

        std::str::from_utf8(bytes).map_err(|e| E!(BadUtf8String(e), err_data, ix))
    }

    pub fn read_array_len_from_tag(&mut self, tag: TagByte) -> Result<usize, KsonError> {
        if tag.val & COLLECTION_IS_MAP == COLLECTION_IS_MAP {
            e!(
                WrongMinorType {
                    expected: "array",
                    found: "map".into()
                },
                self.data.clone(),
                self.ix
            )
        }

        let prelen = tag.val & !(MASK_TYPE | BIG_BIT | COLLECTION_IS_MAP);
        Ok(if !tag.is_big {
            prelen as usize
        } else {
            self.read_raw_u64(prelen)? as usize
        })
    }

    pub fn read_map_len_from_tag(&mut self, tag: TagByte) -> Result<usize, KsonError> {
        if tag.val & COLLECTION_IS_MAP != COLLECTION_IS_MAP {
            e!(
                WrongMinorType {
                    expected: "map",
                    found: "array".into()
                },
                self.data.clone(),
                self.ix
            )
        }

        let prelen = tag.val & !(MASK_TYPE | BIG_BIT | COLLECTION_IS_MAP);
        Ok(if !tag.is_big {
            prelen as usize
        } else {
            self.read_raw_u64(prelen)? as usize
        })
    }

    pub fn read_cons_meta_from_tag(&mut self, tag: TagByte) -> Result<(bool, usize), KsonError> {
        let is_map = tag.val & COLLECTION_IS_MAP == COLLECTION_IS_MAP;
        let prelen = tag.val & !COLLECTION_IS_MAP;
        Ok((
            is_map,
            if !tag.is_big {
                prelen as usize
            } else {
                self.read_raw_u64(prelen + 1)? as usize
            },
        ))
    }

    pub fn read_cons<Car, F, Cdr, G>(
        &mut self,
        car_reader: F,
        cdr_reader: G,
    ) -> Result<Cdr, KsonError>
    where
        F: FnOnce(&mut Self, bool, usize) -> Result<Car, KsonError>,
        G: FnOnce(&mut Self, Car) -> Result<Cdr, KsonError>,
    {
        let tag = self.read_cons_tag()?;
        let (is_map, len) = self.read_cons_meta_from_tag(tag)?;
        let car = car_reader(self, is_map, len)?;
        let cdr = cdr_reader(self, car)?;
        Ok(cdr)
    }

    pub fn check_entry<V: De>(&mut self, key_should_be: &'static str) -> Result<V, KsonError> {
        let err_data = self.data.clone();
        let err_ix = self.ix;
        let key = self.read_str()?;

        if key != key_should_be {
            e!(
                WrongConsKey {
                    expected: key_should_be,
                    found: key.into()
                },
                err_data,
                err_ix
            )
        }

        V::de(self)
    }

    pub fn take_val<V: De>(&mut self) -> Result<V, KsonError> {
        V::de(self)
    }
}

pub fn from_bytes<T: De>(from: Bytes) -> Result<T, KsonError> {
    T::de(&mut Deserializer::new(from))
}

macro_rules! read_tagged_val {
    ($type: ty, $fname: ident, $tag_reader: tt, $val_reader: tt) => {
        impl Deserializer {
            pub fn $fname(&mut self) -> Result<$type, KsonError> {
                let tag = self.$tag_reader()?;
                let val = self.$val_reader(tag)?;
                Ok(val)
            }
        }
    };
}

read_tagged_val!(u8, read_u8, read_uint_tag, read_u8_from_tag);
read_tagged_val!(u16, read_u16, read_uint_tag, read_u16_from_tag);
read_tagged_val!(u32, read_u32, read_uint_tag, read_u32_from_tag);
read_tagged_val!(u64, read_u64, read_uint_tag, read_u64_from_tag);
read_tagged_val!(u128, read_u128, read_uint_tag, read_u128_from_tag);

read_tagged_val!(&str, read_str, read_bytes_tag, read_str_from_tag);
read_tagged_val!(Bytes, read_bytes, read_bytes_tag, read_bytes_from_tag);

impl Deserializer {
    pub fn read_string(&mut self) -> Result<String, KsonError> {
        let borrowed = self.read_str()?;
        Ok(borrowed.into())
    }
}

macro_rules! trivial_de {
    ($type: ty, $mname: ident) => {
        impl De for $type {
            fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
                d.$mname()
            }
        }
    };
}

trivial_de!(u8, read_u8);
trivial_de!(u16, read_u16);
trivial_de!(u32, read_u32);
trivial_de!(u64, read_u64);
trivial_de!(u128, read_u128);
trivial_de!(String, read_string);
trivial_de!(Bytes, read_bytes);

mod __impls {
    use super::*;

    mod __std {
        use super::*;
        use std::collections::*;

        impl<T: De> De for Vec<T> {
            fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
                let tag = d.read_coll_tag()?;
                let len = d.read_array_len_from_tag(tag)?;

                let mut out = Vec::with_capacity(len);
                for _ in 0..len {
                    out.push(T::de(d)?);
                }

                Ok(out)
            }
        }

        impl<T: De> De for LinkedList<T> {
            fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
                let tag = d.read_coll_tag()?;
                let len = d.read_array_len_from_tag(tag)?;

                let mut out = LinkedList::new();
                for _ in 0..len {
                    out.push_back(T::de(d)?);
                }

                Ok(out)
            }
        }

        impl<T: De> De for VecDeque<T> {
            fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
                let tag = d.read_coll_tag()?;
                let len = d.read_array_len_from_tag(tag)?;

                let mut out = VecDeque::with_capacity(len);
                for _ in 0..len {
                    out.push_back(T::de(d)?);
                }

                Ok(out)
            }
        }

        impl<T: De + Ord> De for BTreeSet<T> {
            fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
                let tag = d.read_coll_tag()?;
                let len = d.read_array_len_from_tag(tag)?;

                let mut out = BTreeSet::new();
                for _ in 0..len {
                    out.insert(T::de(d)?);
                }

                Ok(out)
            }
        }

        impl<S: std::hash::BuildHasher + Default, T: De + std::hash::Hash + Eq> De for HashSet<T, S> {
            fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
                let tag = d.read_coll_tag()?;
                let len = d.read_array_len_from_tag(tag)?;

                let mut out = HashSet::with_capacity_and_hasher(len, S::default());
                for _ in 0..len {
                    out.insert(T::de(d)?);
                }

                Ok(out)
            }
        }

        impl<K: De + Ord, V: De> De for BTreeMap<K, V> {
            fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
                let tag = d.read_coll_tag()?;
                let len = d.read_map_len_from_tag(tag)?;

                let mut out = BTreeMap::new();
                for _ in 0..len {
                    let key = K::de(d)?;
                    let val = V::de(d)?;
                    out.insert(key, val);
                }

                Ok(out)
            }
        }

        impl<S: std::hash::BuildHasher + Default, K: De + std::hash::Hash + Eq, V: De> De
            for HashMap<K, V, S>
        {
            fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
                let tag = d.read_coll_tag()?;
                let len = d.read_map_len_from_tag(tag)?;

                let mut out = HashMap::with_capacity_and_hasher(len, S::default());
                for _ in 0..len {
                    let key = K::de(d)?;
                    let val = V::de(d)?;
                    out.insert(key, val);
                }

                Ok(out)
            }
        }
    }

    mod __arrayvec {
        use super::*;
        use arrayvec::*;

        impl<T: De, A: Array<Item = T>> De for ArrayVec<A> {
            fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
                let tag = d.read_coll_tag()?;
                let len = d.read_map_len_from_tag(tag)?;

                if len > A::CAPACITY {
                    e!(
                        CollectionTooLarge {
                            max_len: A::CAPACITY,
                            found: len
                        },
                        d.data.clone(),
                        d.ix
                    )
                }

                let mut out = ArrayVec::new();
                for _ in 0..len {
                    out.push(T::de(d)?);
                }

                Ok(out)
            }
        }
    }
}
