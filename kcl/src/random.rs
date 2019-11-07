use libsodium_sys::*;

pub fn gen_into(buf: &mut [u8]) {
    unsafe { randombytes_buf(buf.as_mut_ptr() as _, buf.len()) }
}
