use kson_strategy::*;
use proptest::prelude::*;

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

proptest! {
    #![proptest_config(ProptestConfig { cases: 100, ..ProptestConfig::default() })]

    #[test]
    fn encode_decode_val(k in arb_value(65, 65, 100)) {
        let enc = kson::to_vec(&k);
        let dec = kson::from_bytes(enc.into()).expect("failed to parse");
        assert_eq!(k, dec);
    }
}
