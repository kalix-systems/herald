use kson::{prelude::*, value::*};
use proptest::prelude::*;
use std::fmt::Debug;

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

// const VEC_LEN: usize = 257;
// const MAP_LEN: usize = 257;

pub fn arb_coll<T, K, V, St, Sk, Sv>(
    vec_len: usize,
    map_len: usize,
    st: St,
    sk: Sk,
    sv: Sv,
) -> impl Strategy<Value = Collection<T, K, V>>
where
    St: Strategy<Value = T>,
    Sk: Strategy<Value = K>,
    Sv: Strategy<Value = V>,
    T: Debug,
    K: Debug + Ord,
    V: Debug,
{
    prop_oneof![
        prop::collection::vec(st, 0..vec_len).prop_map(Collection::Arr),
        prop::collection::btree_map(sk, sv, 0..map_len).prop_map(Collection::Map)
    ]
}

pub fn arb_atomic_coll() -> impl Strategy<Value = Collection<Atom, Atom, Atom>> {
    arb_coll(257, 257, arb_atom(), arb_atom(), arb_atom())
}

pub fn arb_value(
    max_depth: usize,
    max_width: usize,
    max_nodes: usize,
) -> impl Strategy<Value = Value> {
    let leaf = arb_atom().prop_map(Value::Atom);
    leaf.prop_recursive(
        max_depth as u32, // max depth
        max_nodes as u32, // max nodes
        max_width as u32, // max items per collection
        move |inner| {
            prop_oneof![
                arb_coll(
                    max_width,
                    max_width,
                    inner.clone(),
                    inner.clone(),
                    inner.clone()
                )
                .prop_map(Value::Collection),
                (
                    inner.clone(),
                    arb_coll(
                        max_width,
                        max_width,
                        inner.clone(),
                        inner.clone(),
                        inner.clone()
                    )
                )
                    .prop_map(|(t, c)| Value::Cons(Box::new(t), c))
            ]
        },
    )
}
