use bytes::Bytes;
use kson::{prelude::*, value::*};
use proptest::prelude::*;

/// arbitrary Bytes for use with proptest
pub fn arb_bs() -> impl Strategy<Value = Bytes> {
    ".*".prop_map(|s| -> Bytes { Bytes::from(s) })
}

/// arbitrary KSON for use with proptest
pub fn arb_atom() -> impl Strategy<Value = Atom> {
    prop_oneof![
        Just(Atom::Null),
        // misc
        any::<bool>().prop_map(Atom::Bool),
        // integers
        any::<u128>().prop_map(Atom::UInt),
        any::<i128>().prop_map(Atom::Int),
        // bytestrings
        arb_bs().prop_map(Atom::Bytes),
        // unicode strings
        any::<String>().prop_map(Atom::String),
    ]
}

const VEC_LEN: usize = 257;
const MAP_LEN: usize = 257;
pub fn arb_atomic_coll() -> impl Strategy<Value = Collection<Atom, Atom, Atom>> {
    prop_oneof![
        prop::collection::vec(arb_atom(), 0..VEC_LEN).prop_map(Collection::Arr),
        prop::collection::btree_map(arb_atom(), arb_atom(), 0..MAP_LEN).prop_map(Collection::Map)
    ]
}

// pub fn arb_value() -> impl Strategy<Value = Value> {
// }

proptest! {
    #![proptest_config(ProptestConfig { cases: 10_000, ..ProptestConfig::default() })]

    #[test]
    fn encode_decode_atom(k in arb_atom()) {
        let enc = kson::to_vec(&k);
        let dec = kson::from_bytes(enc.into()).expect("failed to parse");
        assert_eq!(k, dec);
    }
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 100, ..ProptestConfig::default() })]

    #[test]
    fn encode_decode_atomic_coll(k in arb_atomic_coll()) {
        let enc = kson::to_vec(&k);
        let dec = kson::from_bytes(enc.into()).expect("failed to parse");
        assert_eq!(k, dec);
    }
}
