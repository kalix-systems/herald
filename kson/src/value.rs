use bytes::Bytes;

pub enum UInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}
pub enum Int {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
}

pub enum Atom {
    Bool(bool),
    UInt(UInt),
    Int(Int),
    Bytes(Bytes),
    String(String),
}

pub enum Value {
    Array(Vec<Atom>),
    Cons(Atom, Vec<Value>),
    Map(Vec<(Atom, Value)>),
}
