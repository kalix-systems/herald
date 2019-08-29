#[macro_use]
extern crate criterion;
use criterion::{black_box, Criterion};

use stupid_crypto::kem::*;

fn time_encapsulate(c: &mut Criterion) {
    c.bench_function("generate a shared secret", |b| {
        let pair = Pair::new();
        b.iter(move || black_box(pair.pub_key()).encapsulate())
    });
}

fn time_decapsulate(c: &mut Criterion) {
    c.bench_function("recover a shared secret", |b| {
        let pair = Pair::new();
        let capsule = pair.pub_key().encapsulate();
        b.iter(move || {
            let ss = pair.sec_key().decapsulate(black_box(capsule.cipher()));
            assert!(capsule.shared() == &ss);
        })
    });
}

criterion_group!(ntru, time_encapsulate, time_decapsulate);
criterion_main!(ntru);
