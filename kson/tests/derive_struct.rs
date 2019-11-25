use bytes::Bytes;
use kson::*;

#[derive(Eq, PartialEq, Debug, Ser, De)]
pub struct UnitLike;

#[test]
fn unit_like_serde() {
    let val = UnitLike;
    let as_vec = kson::to_vec(&val);
    let val2 = kson::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(val, val2);
}

#[derive(Eq, PartialEq, Debug, Ser, De)]
pub struct EmptyTuple();

#[test]
fn empty_tuple_serde() {
    let val = EmptyTuple();
    let as_vec = kson::to_vec(&val);
    let val2 = kson::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(val, val2);
}

#[derive(Eq, PartialEq, Debug, Ser, De)]
pub struct Newtype(i64);

#[test]
fn newtype_serde() {
    let val = Newtype(i64::min_value());
    let as_vec = kson::to_vec(&val);
    let val2 = kson::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(val, val2);

    let val = Newtype(i64::max_value());
    let as_vec = kson::to_vec(&val);
    let val2 = kson::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(val, val2);
}

#[derive(Eq, PartialEq, Debug, Ser, De)]
pub struct Tuple(u8, u16, u32, u64, u128);

#[test]
fn tuple_serde() {
    let val = Tuple(
        u8::max_value(),
        u16::max_value(),
        u32::max_value(),
        u64::max_value(),
        u128::max_value(),
    );
    let as_vec = kson::to_vec(&val);
    let val2 = kson::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(val, val2);
}

#[derive(Eq, PartialEq, Debug, Ser, De)]
pub struct Struct {
    first: u8,
    second: u16,
    third: u32,
    fourth: u64,
    fifth: u128,
}

#[test]
fn struct_serde() {
    let val = Struct {
        first: u8::max_value(),
        second: u16::max_value(),
        third: u32::max_value(),
        fourth: u64::max_value(),
        fifth: u128::max_value(),
    };
    let as_vec = kson::to_vec(&val);
    let val2 = kson::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(val, val2);
}

#[derive(Eq, PartialEq, Debug, Ser, De)]
pub struct Generic<T1, T2> {
    in1: T1,
    in2: T2,
    rest: Bytes,
}

#[test]
fn generic_serde() {
    let mut map = std::collections::BTreeMap::new();
    map.insert(0u8, 0u8);
    map.insert(1u8, 0u8);
    let val = Generic {
        in1: vec![0u8],
        in2: map,
        rest: Bytes::from_static(b"asdf"),
    };
    let as_vec = kson::to_vec(&val);
    let val2 = kson::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(val, val2);
}
