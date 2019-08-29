#[macro_use]
extern crate criterion;
use criterion::{black_box, Criterion};

use stupid_crypto::sig::*;

const MSG: &'static [u8] = &[0; 256];

fn time_sign(c: &mut Criterion) {
    c.bench_function(&format!("sign a message of length {}", MSG.len()), |b| {
        let pair = Pair::new();
        b.iter(move || black_box(pair.sec_key().sign(black_box(MSG))))
    });
}

fn time_verify(c: &mut Criterion) {
    c.bench_function(
        &format!("verify signature for a message of length {}", MSG.len()),
        |b| {
            let pair = Pair::new();
            let sig = pair.sec_key().sign(MSG);
            b.iter(move || assert!(pair.pub_key().verify(black_box(MSG), black_box(&sig))))
        },
    );
}

criterion_group!(falcon, time_sign, time_verify);
criterion_main!(falcon);
