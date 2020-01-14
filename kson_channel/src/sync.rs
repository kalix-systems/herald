use super::*;
use crate::error::FramedError;
use std::io::{Read, Write};

macro_rules! read_write_u {
    ($read_name:ident, $write_name:ident, $ty:ident, $len:expr) => {
        impl<S: Write> Framed<S> {
            pub fn $write_name(
                &mut self,
                u: $ty,
            ) -> Result<(), FramedError> {
                self.inner.write_all(&u.to_le_bytes())?;
                Ok(())
            }
        }

        impl<S: Read> Framed<S> {
            pub fn $read_name(&mut self) -> Result<$ty, FramedError> {
                let mut buf = [0u8; $len];
                self.inner.read_exact(&mut buf)?;
                Ok($ty::from_le_bytes(buf))
            }
        }
    };
}

read_write_u!(read_u8_sync, write_u8_sync, u8, 1);
read_write_u!(read_u16_sync, write_u16_sync, u16, 2);
read_write_u!(read_u32_sync, write_u32_sync, u32, 4);
read_write_u!(read_u64_sync, write_u64_sync, u64, 8);
read_write_u!(read_u128_sync, write_u128_sync, u128, 16);

impl<W: Write> Framed<W> {
    pub fn write_ser_sync<T: Ser>(
        &mut self,
        t: &T,
    ) -> Result<(), FramedError> {
        let msg = kson::to_vec(t);
        self.write_u32_sync(msg.len() as u32)?;
        self.inner.write_all(&msg)?;
        Ok(())
    }
}

impl<R: Read> Framed<R> {
    pub fn read_de_sync<T>(&mut self) -> Result<T, FramedError>
    where
        T: De,
    {
        let len = self.read_u32_sync()?;
        let mut buf = vec![0u8; len as usize];
        self.inner.read_exact(&mut buf)?;
        let res = kson::from_bytes(buf.into())?;
        Ok(res)
    }
}
