use crate::new_type;

use arrayvec::ArrayVec;
use libsodium_sys::*;

pub const KEY_LEN: usize = crypto_generichash_blake2b_KEYBYTES as usize;
pub const HASH_MIN_LEN: usize = crypto_generichash_blake2b_BYTES_MIN as usize;
pub const HASH_MAX_LEN: usize = crypto_generichash_blake2b_BYTES_MAX as usize;
pub const HASH_REC_LEN: usize = crypto_generichash_blake2b_BYTES as usize;

pub fn simple_hash(msg: &[u8]) -> [u8; HASH_REC_LEN] {
    let mut buf = [0u8; HASH_REC_LEN];
    let res = unsafe {
        crypto_generichash_blake2b(
            buf.as_mut_ptr(),
            HASH_REC_LEN,
            msg.as_ptr(),
            msg.len() as _,
            std::ptr::null(),
            0,
        )
    };
    assert_eq!(res, 0);
    buf
}

pub fn simple_hash_into(
    buf: &mut [u8],
    msg: &[u8],
) {
    assert!(HASH_MIN_LEN <= buf.len());
    assert!(buf.len() <= HASH_MAX_LEN);
    let res = unsafe {
        crypto_generichash_blake2b(
            buf.as_mut_ptr(),
            HASH_REC_LEN,
            msg.as_ptr(),
            msg.len() as _,
            std::ptr::null(),
            0,
        )
    };
    assert_eq!(res, 0);
}

new_type! {
    secret Key(KEY_LEN)
}

impl Key {
    pub fn new() -> Self {
        let mut buf = [0u8; KEY_LEN];
        crate::random::gen_into(&mut buf);
        Key(buf)
    }

    pub fn hash_into(
        &self,
        buf: &mut [u8],
        msg: &[u8],
    ) {
        assert!(HASH_MIN_LEN <= buf.len());
        assert!(buf.len() <= HASH_MAX_LEN);
        let result = unsafe {
            crypto_generichash_blake2b(
                buf.as_mut_ptr(),
                buf.len(),
                msg.as_ptr(),
                msg.len() as u64,
                self.0.as_ptr(),
                self.0.len(),
            )
        };
        assert_eq!(result, 0);
    }

    pub fn hash_into_many<'a, I: IntoIterator<Item = &'a mut [u8]>>(
        &self,
        msg: &[u8],
        keys: I,
    ) {
        for (i, key_buf) in keys.into_iter().enumerate() {
            let mut hasher = Builder::new().out_len(key_buf.len()).key(self).build();
            hasher.update(msg);
            hasher.update(&u64::to_le_bytes(i as u64));
            hasher.finalize_into(key_buf);
        }
    }
}

pub struct Builder<'a> {
    out_len: Option<usize>,
    key: Option<&'a Key>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Builder {
            out_len: None,
            key: None,
        }
    }

    pub fn out_len(
        mut self,
        out_len: usize,
    ) -> Self {
        self.out_len = Some(out_len);
        self
    }

    pub fn key(
        mut self,
        key: &'a Key,
    ) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self) -> Hasher {
        Hasher::mk(self.out_len.unwrap_or(HASH_REC_LEN), self.key)
    }
}

pub struct Digest(pub ArrayVec<[u8; HASH_MAX_LEN]>);

pub struct Hasher {
    out_len: usize,
    state: crypto_generichash_blake2b_state,
}

impl Hasher {
    fn mk(
        out_len: usize,
        key: Option<&Key>,
    ) -> Self {
        assert!(HASH_MIN_LEN <= out_len);
        assert!(out_len <= HASH_MAX_LEN);

        let mut state = std::mem::MaybeUninit::uninit();

        let (key_ptr, key_len) = key
            .map(|k| (k.0.as_ptr(), k.0.len()))
            .unwrap_or((std::ptr::null(), 0));

        let result = unsafe {
            crypto_generichash_blake2b_init(state.as_mut_ptr(), key_ptr, key_len, out_len)
        };
        assert_eq!(result, 0);

        let state = unsafe { state.assume_init() };
        Hasher { out_len, state }
    }

    pub fn update(
        &mut self,
        data: &[u8],
    ) {
        let res = unsafe {
            crypto_generichash_blake2b_update(&mut self.state, data.as_ptr(), data.len() as u64)
        };
        assert_eq!(res, 0);
    }

    pub fn finalize(mut self) -> Digest {
        let mut buf = ArrayVec::new();
        unsafe {
            let res =
                crypto_generichash_blake2b_final(&mut self.state, buf.as_mut_ptr(), self.out_len);
            assert_eq!(res, 0);
            buf.set_len(self.out_len);
        };
        Digest(buf)
    }

    pub fn finalize_into(
        mut self,
        buf: &mut [u8],
    ) {
        assert!(buf.len() == self.out_len);
        unsafe {
            let res =
                crypto_generichash_blake2b_final(&mut self.state, buf.as_mut_ptr(), buf.len());
            assert_eq!(res, 0);
        };
    }
}

impl Drop for Hasher {
    fn drop(&mut self) {
        let as_ptr = (&mut self.state) as *mut crypto_generichash_blake2b_state;
        unsafe {
            let len = crypto_generichash_blake2b_statebytes();
            sodium_memzero(as_ptr as *mut std::ffi::c_void, len as usize);
        }
    }
}
