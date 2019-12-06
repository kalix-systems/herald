#[cfg(feature = "proptest")]
mod __ {
    use kson::strategy::*;
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
        fn encode_decode_val(k in arb_value(20, 20, 20)) {
            let enc = kson::to_vec(&k);
            let dec = kson::from_bytes(enc.into()).expect("failed to parse");
            assert_eq!(k, dec);
        }
    }
}
