use libsodium_sys::*;

pub fn gen_into(buf: &mut [u8]) {
    unsafe { randombytes_buf(buf.as_mut_ptr() as _, buf.len()) }
}

new_type! {
    /// A unique identifier
    public UQ(32)
}

impl UQ {
    /// Generate a new `[UQ]`. Guaranteed never to collide with another instance.
    pub fn gen_new() -> Self {
        let mut buf = [0u8; 32];
        gen_into(&mut buf);
        UQ(buf)
    }
}
