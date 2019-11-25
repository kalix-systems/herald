use crate::prelude::*;
use bytes::Bytes;
use std::convert::TryInto;

#[derive(Debug, Eq, PartialEq, Clone)]
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
            _ => unimplemented!(),
        };

        Ok(res)
    }
}

pub enum Value {
    Atom(Atom),
    Array(Vec<Value>),
    Cons(Box<Value>, Vec<Value>),
    Map(Vec<(Value, Value)>),
}
