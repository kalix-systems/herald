use crate::prelude::*;
use bytes::Bytes;
use std::{collections::BTreeMap, convert::TryInto};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum Atom {
    Null,
    Bool(bool),
    UInt(u128),
    Int(i128),
    Bytes(Bytes),
    String(String),
}

impl Ser for Atom {
    fn ser(
        &self,
        into: &mut Serializer,
    ) {
        use Atom::*;
        match self {
            Null => ().ser(into),
            Bool(b) => b.ser(into),
            UInt(u) => u.ser(into),
            Int(i) => i.ser(into),
            Bytes(b) => b.ser(into),
            String(s) => s.ser(into),
        }
    }
}

impl De for Atom {
    fn de(from: &mut Deserializer) -> Result<Self, KsonError> {
        let TagByteWithType { typ, is_big, val } = from.read_tag_byte()?;
        let tag = TagByte { is_big, val };

        let res = match typ {
            Type::Special => match tag.val.try_into() {
                Ok(Constants::False) => Atom::Bool(false),
                Ok(Constants::True) => Atom::Bool(true),
                Ok(Constants::Null) => Atom::Null,
                Err(o) => {
                    e!(UnknownConst(o), from.data.clone(), from.ix);
                }
            },
            Type::Unsigned => Atom::UInt(from.read_u128_from_tag(tag)?),
            Type::Signed => Atom::Int(from.read_i128_from_tag(tag)?),
            Type::Bytes => {
                if tag.val & BYTES_ARE_UTF8 == BYTES_ARE_UTF8 {
                    Atom::String(from.read_str_from_tag(tag)?.into())
                } else {
                    Atom::Bytes(from.read_bytes_from_tag(tag)?)
                }
            }
            t => e!(
                CustomError(format!("can't deserialize type {:?} as an atom", t)),
                from.data.clone(),
                from.ix
            ),
        };

        Ok(res)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum Collection<T, K: Ord, V> {
    Arr(Vec<T>),
    Map(BTreeMap<K, V>),
}

impl<T: Ser, K: Ser + Ord, V: Ser> Ser for Collection<T, K, V> {
    fn ser(
        &self,
        into: &mut Serializer,
    ) {
        match self {
            Collection::Arr(a) => a.ser(into),
            Collection::Map(m) => m.ser(into),
        }
    }
}

impl<T: De, K: De + Ord, V: De> De for Collection<T, K, V> {
    fn de(from: &mut Deserializer) -> Result<Self, KsonError> {
        let tag = from.read_coll_tag()?;

        // this is a stupid hack, but I'm trying to make this a good test not to make it fast
        from.ix -= 1;

        if tag.val & COLLECTION_IS_MAP == COLLECTION_IS_MAP {
            Ok(Collection::Map(from.take_val()?))
        } else {
            Ok(Collection::Arr(from.take_val()?))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum Value {
    Atom(Atom),
    Collection(Collection<Value, Value, Value>),
    Cons(Box<Value>, Collection<Value, Value, Value>),
}

// impl Ser for Value {
//     fn ser(&self, into: &mut Serializer) {
//         match self {
//             Atom(a) => a.ser(into),
//             Array(v) => v.ser(into),
//             Cons(t, r) =>
//             Map(m)
//         }
//     }
// }
