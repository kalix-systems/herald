use bytes::Bytes;
use kson::*;

#[derive(Eq, PartialEq, Debug, Ser, De)]
pub enum Variants {
    Unit,
    Tuple(u8, u16, u32, u64, u128),
    Named {
        first: u8,
        second: u16,
        third: u32,
        fourth: u64,
        fifth: u128,
    },
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
    };
    let as_vec = kson::ser::into_vec(&val);
    let val2 = kson::de::from_bytes(Bytes::from(as_vec)).expect("failed to deserialize");
    assert_eq!(val, val2);
}
