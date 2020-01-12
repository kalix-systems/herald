#![allow(clippy::new_without_default)]

#[macro_use]
mod newtype_macros;

pub mod aead;
pub mod box_;
pub mod ed25519;
pub mod hash;
pub mod kx;
pub mod random;
pub mod sign;
pub mod x25519;

pub(crate) use libsodium_sys as ffi;
pub use libsodium_sys;

use std::sync::{
    atomic::{AtomicI32, Ordering},
    Once,
};

static IS_INIT: AtomicI32 = AtomicI32::new(0);
static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| unsafe {
        let res = libsodium_sys::sodium_init();
        IS_INIT.store(res, Ordering::Release);
    });
    assert_eq!(IS_INIT.load(Ordering::Acquire), 0);
}
