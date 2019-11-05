use super::{utils::*, *};
use bytes::Bytes;

#[derive(Default)]
pub struct Serializer(pub Vec<u8>);

impl Serializer {
    pub fn new() -> Self {
        Serializer(Vec::new())
    }
}

pub trait Ser {
    fn ser(&self, into: &mut Serializer);
}

pub trait AtomicSer: Ser {}

macro_rules! write_uint {
    ($fname: ident, $ty: tt, $digs: ident) => {
        impl Serializer {
            pub fn $fname(&mut self, u: $ty) {
                let typ = Type::Unsigned as u8;
                if u == 0 {
                    self.0.push(typ);
                } else if u < BIG_BIT as $ty {
                    let byt = u as u8;
                    self.0.push(typ | byt);
                } else {
                    let bytes = $digs(u);
                    let ulen = bytes.len() - 1;
                    debug_assert!(ulen < 16);
                    self.0.push(typ | BIG_BIT | (ulen as u8));
                    self.0.extend_from_slice(&bytes);
                }
            }
        }
    };
}

write_uint!(write_u8, u8, bytes_of_u8);
write_uint!(write_u16, u16, bytes_of_u16);
write_uint!(write_u32, u32, bytes_of_u32);
write_uint!(write_u64, u64, bytes_of_u64);
write_uint!(write_u128, u128, bytes_of_u128);

macro_rules! write_int {
    ($fname: ident, $ty: tt, $ity: tt) => {
        impl Serializer {
            pub fn $fname(&mut self, i: $ty) {
                let typ = Type::Signed as u8;
                self.0.push(typ | SignedType::$ity as u8);
                let digs = $ty::to_le_bytes(i);
                self.0.extend_from_slice(&digs);
            }
        }
    };
}

write_int!(write_i8, i8, I8);
write_int!(write_i16, i16, I16);
write_int!(write_i32, i32, I32);
write_int!(write_i64, i64, I64);
write_int!(write_i128, i128, I128);

impl Serializer {
    pub fn write_bool(&mut self, b: bool) {
        self.0.push(if b { 1 } else { 0 })
    }

    fn write_slice(&mut self, is_utf8: bool, raw: &[u8]) {
        debug_assert!(!is_utf8 | std::str::from_utf8(raw).is_ok());

        let major_type = Type::Bytes as u8;
        let minor_type = if is_utf8 { BYTES_ARE_UTF8 } else { 0 };
        let mut tag = major_type | minor_type;

        if raw.len() < BYTES_ARE_UTF8 as usize {
            tag |= raw.len() as u8;
            self.0.push(tag);
        } else {
            tag |= BIG_BIT;

            let digs = bytes_of_u64(raw.len() as u64);
            debug_assert!(digs.len() <= BYTES_ARE_UTF8 as usize);
            tag |= digs.len() as u8 - 1;

            self.0.push(tag);
            self.0.extend_from_slice(&digs);
        }

        self.0.extend_from_slice(raw);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.write_slice(false, bytes)
    }

    pub fn write_string(&mut self, string: &str) {
        self.write_slice(true, string.as_bytes())
    }

    pub fn start_vec(&mut self, len: usize) {
        let major_type = Type::Collection as u8;
        let minor_type = 0;
        let mut tag = major_type | minor_type;
        if len < COLLECTION_IS_MAP as usize {
            tag |= len as u8;
            self.0.push(tag);
        } else {
            tag |= BIG_BIT;

            let digs = bytes_of_u64(len as u64 - 1);
            debug_assert!(digs.len() < BYTES_ARE_UTF8 as usize);
            tag |= digs.len() as u8;

            self.0.push(tag);
            self.0.extend_from_slice(&digs);
        }
    }

    pub fn put_vec_item<T: Ser + ?Sized>(&mut self, item: &T) {
        item.ser(self);
    }

    pub fn write_vec<'a, T, I>(&mut self, items: I)
    where
        T: 'a + Ser + ?Sized,
        I: ExactSizeIterator + Iterator<Item = &'a T>,
    {
        self.start_vec(items.len());
        for item in items {
            self.put_vec_item(item);
        }
    }

    pub fn start_map(&mut self, len: usize) {
        let major_type = Type::Collection as u8;
        let minor_type = COLLECTION_IS_MAP;
        let mut tag = major_type | minor_type;
        if len < COLLECTION_IS_MAP as usize {
            tag |= len as u8;
            self.0.push(tag);
        } else {
            tag |= BIG_BIT;

            let digs = bytes_of_u64(len as u64 - 1);
            debug_assert!(digs.len() < BYTES_ARE_UTF8 as usize);
            tag |= digs.len() as u8;

            self.0.push(tag);
            self.0.extend_from_slice(&digs);
        }
    }

    pub fn put_map_pair<K: AtomicSer + ?Sized, V: Ser + ?Sized>(&mut self, key: &K, val: &V) {
        key.ser(self);
        val.ser(self);
    }

    pub fn write_map<'a, K, V, I>(&mut self, items: I)
    where
        K: 'a + Ser + AtomicSer + ?Sized,
        V: 'a + Ser + ?Sized,
        I: ExactSizeIterator + Iterator<Item = (&'a K, &'a V)>,
    {
        self.start_map(items.len());

        for (k, v) in items {
            self.put_map_pair(k, v);
        }
    }

    pub fn start_cons(&mut self, is_map: bool, len: usize) {
        let major_type = Type::Cons as u8;
        let minor_type = if is_map { COLLECTION_IS_MAP } else { 0 };
        let mut tag = major_type | minor_type;
        if len < COLLECTION_IS_MAP as usize {
            tag |= len as u8;
            self.0.push(tag);
        } else {
            tag |= BIG_BIT;

            let digs = bytes_of_u64(len as u64);
            debug_assert!(digs.len() <= BIG_BIT as usize);
            tag |= digs.len() as u8 - 1;

            self.0.push(tag);
            self.0.extend_from_slice(&digs);
        }
    }

    pub fn put_cons_tag<T: AtomicSer + ?Sized>(&mut self, item: &T) {
        item.ser(self);
    }

    pub fn put_cons_item<T: Ser + ?Sized>(&mut self, item: &T) {
        item.ser(self);
    }

    pub fn put_cons_pair<K: AtomicSer + ?Sized, V: Ser + ?Sized>(&mut self, key: &K, val: &V) {
        key.ser(self);
        val.ser(self);
    }
}

macro_rules! trivial_ser_copy {
    ($ty: tt, $method: tt) => {
        impl Ser for $ty {
            fn ser(&self, into: &mut Serializer) {
                into.$method(*self);
            }
        }
    };
}

trivial_ser_copy!(bool, write_bool);

trivial_ser_copy!(u8, write_u8);
trivial_ser_copy!(u16, write_u16);
trivial_ser_copy!(u32, write_u32);
trivial_ser_copy!(u64, write_u64);
trivial_ser_copy!(u128, write_u128);

trivial_ser_copy!(i8, write_i8);
trivial_ser_copy!(i16, write_i16);
trivial_ser_copy!(i32, write_i32);
trivial_ser_copy!(i64, write_i64);
trivial_ser_copy!(i128, write_i128);

macro_rules! trivial_ser {
    ($ty: tt, $method: tt) => {
        impl Ser for $ty {
            fn ser(&self, into: &mut Serializer) {
                into.$method(self);
            }
        }
    };
}

trivial_ser!([u8], write_bytes);
trivial_ser!(Bytes, write_bytes);

trivial_ser!(str, write_string);
trivial_ser!(String, write_string);

impl AtomicSer for bool {}

impl AtomicSer for u8 {}
impl AtomicSer for u16 {}
impl AtomicSer for u32 {}
impl AtomicSer for u64 {}
impl AtomicSer for u128 {}

impl AtomicSer for i8 {}
impl AtomicSer for i16 {}
impl AtomicSer for i32 {}
impl AtomicSer for i64 {}
impl AtomicSer for i128 {}

impl AtomicSer for str {}
impl AtomicSer for String {}

impl AtomicSer for [u8] {}
impl AtomicSer for Bytes {}

pub fn into_vec<T: Ser + ?Sized>(t: &T) -> Vec<u8> {
    let mut out = Serializer::new();
    t.ser(&mut out);
    out.0
}

mod __impls {
    use super::*;

    mod __std {
        use super::*;
        use std::collections::*;

        impl<T: Ser> Ser for Vec<T> {
            fn ser(&self, s: &mut Serializer) {
                s.start_vec(self.len());
                for i in self {
                    i.ser(s);
                }
            }
        }

        impl<T: Ser> Ser for VecDeque<T> {
            fn ser(&self, s: &mut Serializer) {
                s.start_vec(self.len());
                for i in self {
                    i.ser(s);
                }
            }
        }

        impl<T: Ser> Ser for BinaryHeap<T> {
            fn ser(&self, s: &mut Serializer) {
                s.start_vec(self.len());
                for i in self {
                    i.ser(s);
                }
            }
        }

        impl<T: Ser> Ser for LinkedList<T> {
            fn ser(&self, s: &mut Serializer) {
                s.start_vec(self.len());
                for i in self {
                    i.ser(s);
                }
            }
        }

        impl<K: AtomicSer, V: Ser, S: std::hash::BuildHasher> Ser for HashMap<K, V, S> {
            fn ser(&self, s: &mut Serializer) {
                s.start_map(self.len());
                for (k, v) in self {
                    s.put_map_pair(k, v);
                }
            }
        }

        impl<K: AtomicSer, V: Ser> Ser for BTreeMap<K, V> {
            fn ser(&self, s: &mut Serializer) {
                s.start_map(self.len());
                for (k, v) in self {
                    s.put_map_pair(k, v);
                }
            }
        }

        impl<T: Ser, S: std::hash::BuildHasher> Ser for HashSet<T, S> {
            fn ser(&self, s: &mut Serializer) {
                s.start_vec(self.len());
                for t in self {
                    t.ser(s);
                }
            }
        }

        impl<T: Ser> Ser for BTreeSet<T> {
            fn ser(&self, s: &mut Serializer) {
                s.start_vec(self.len());
                for t in self {
                    t.ser(s);
                }
            }
        }
    }

    mod __arrayvec {
        use super::*;
        use arrayvec::*;

        impl<T: Ser, A: Array<Item = T>> Ser for ArrayVec<A> {
            fn ser(&self, s: &mut Serializer) {
                s.start_vec(self.len());

                for t in self {
                    t.ser(s);
                }
            }
        }

        impl<A: Array<Item = u8> + Copy> Ser for ArrayString<A> {
            fn ser(&self, s: &mut Serializer) {
                s.write_string(self.as_str());
            }
        }

        impl<A: Array<Item = u8> + Copy> AtomicSer for ArrayString<A> {}
    }

    mod __ptr {
        use super::*;
        use std::{rc::Rc, sync::Arc};

        macro_rules! ptr_impl {
            ($pt:ident, $($pts:tt),*) => {
                ptr_impl!($pt);
                ptr_impl!($($pts),*);
            };
            ($pt: ident) => {
                impl<T: Ser + ?Sized> Ser for $pt<T> {
                    fn ser(&self, s: &mut Serializer) {
                        <T as Ser>::ser(self, s);
                    }
                }
            };
        }

        ptr_impl!(Box, Arc, Rc);

        impl<T: Ser + ?Sized> Ser for &T {
            fn ser(&self, s: &mut Serializer) {
                <T as Ser>::ser(self, s);
            }
        }
        impl<T: Ser> Ser for &mut T {
            fn ser(&self, s: &mut Serializer) {
                <T as Ser>::ser(self, s);
            }
        }
    }
}
