use arrayvec::*;

macro_rules! bytes_of_uint {
    ($name: ident, $ty: tt, $len: tt) => {
        pub fn $name(u: $ty) -> ArrayVec<[u8; $len]> {
            let len = $len - $ty::leading_zeros(u) as usize / 8;
            let bytes = $ty::to_le_bytes(u);
            let mut out = ArrayVec::new();
            debug_assert!(len <= bytes.len());
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out.as_mut_ptr(), len);
                out.set_len(len)
            }
            out
        }
    };
}

bytes_of_uint!(bytes_of_u8, u8, 1);
bytes_of_uint!(bytes_of_u16, u16, 2);
bytes_of_uint!(bytes_of_u32, u32, 4);
bytes_of_uint!(bytes_of_u64, u64, 8);
bytes_of_uint!(bytes_of_u128, u128, 16);
