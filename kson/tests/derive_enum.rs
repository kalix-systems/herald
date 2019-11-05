use bytes::Bytes;
use kson::*;

#[derive(Eq, PartialEq, Debug, Ser, De)]
pub enum Variants {
    Unit,
    Tuple(u8, u16, u32, u64, u128, Bytes),
    Named {
        first: u8,
        second: u16,
        third: u32,
        fourth: u64,
        fifth: u128,
        last: Bytes,
    },
    Recursive(Vec<Variants>),
}

#[test]
fn unit_like_serde() {
    let val = Variants::Unit;
    let as_vec = kson::ser::into_vec(&val);
    let val2 = kson::de::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(val, val2);
}

#[test]
fn tuple_like_serde() {
    let val = Variants::Tuple(
        u8::max_value(),
        u16::max_value(),
        u32::max_value(),
        u64::max_value(),
        u128::max_value(),
        Bytes::from_static(b"asdf"),
    );
    let as_vec = kson::ser::into_vec(&val);
    let val2 = kson::de::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(val, val2);
}

#[test]
fn struct_like_serde() {
    let val = Variants::Named {
        first: u8::max_value(),
        second: u16::max_value(),
        third: u32::max_value(),
        fourth: u64::max_value(),
        fifth: u128::max_value(),
        last: Bytes::from_static(b"asdf"),
    };
    let as_vec = kson::ser::into_vec(&val);
    let val2 = kson::de::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(val, val2);
}

#[test]
fn recursive_serde() {
    let v1 = Variants::Unit;
    let v2 = Variants::Tuple(
        u8::max_value(),
        u16::max_value(),
        u32::max_value(),
        u64::max_value(),
        u128::max_value(),
        Bytes::from_static(b"asdf"),
    );
    let v3 = Variants::Named {
        first: u8::max_value(),
        second: u16::max_value(),
        third: u32::max_value(),
        fourth: u64::max_value(),
        fifth: u128::max_value(),
        last: Bytes::from_static(b"asdf"),
    };
    let variants: Vec<_> = vec![v1, v2, v3];
    let as_vec = kson::ser::into_vec(&variants);
    let de: Vec<_> = kson::de::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(variants, de);
}

use std::collections::BTreeMap;

#[derive(Eq, PartialEq, Debug, Ser, De)]
pub enum Generic<K: ::kson::ser::AtomicSer + Ord, V> {
    First(K),
    Second(V),
    Map(BTreeMap<K, V>),
}

#[test]
fn generic_serde() {
    type G = Generic<Bytes, u8>;

    let v1 = G::Map(BTreeMap::new());
    let as_vec = kson::ser::into_vec(&v1);
    let v2 = kson::de::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(v1, v2);

    let mut map = std::collections::BTreeMap::new();
    map.insert(Bytes::from_static(b"a"), 0u8);
    map.insert(Bytes::from_static(b""), 0u8);

    let v1 = G::Map(map);
    let as_vec = kson::ser::into_vec(&v1);
    let v2 = kson::de::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(v1, v2);

    let v1 = G::First(Bytes::from_static(b"a"));
    let as_vec = kson::ser::into_vec(&v1);
    let v2 = kson::de::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(v1, v2);

    let v1 = G::Second(0u8);
    let as_vec = kson::ser::into_vec(&v1);
    let v2 = kson::de::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(v1, v2);
}
