pub fn kdf_derive(key: &[u8; 32], subkey_id: u64, ctx: u64, out: &mut [u8]) {
    // TODO: replace this with sodiumoxide bindings when they merge pull request, push to crates.io
    unsafe {
        let r = libsodium_sys::crypto_kdf_blake2b_derive_from_key(
            out.as_mut_ptr(),
            out.len(),
            subkey_id,
            (u64::to_le_bytes(ctx)).as_ptr() as *const i8,
            key.as_ptr(),
        );
        assert_eq!(r, 0, "blake2b kdf failed");
    }
}
