use super::*;
use tokio::prelude::*;

impl<S: AsyncRead + AsyncWrite> Framed<S> {
    pub fn split(
        self
    ) -> (
        Framed<tokio::io::ReadHalf<S>>,
        Framed<tokio::io::WriteHalf<S>>,
    ) {
        let (rx, tx) = tokio::io::split(self.inner);
        (Framed { inner: rx }, Framed { inner: tx })
    }

    pub fn unsplit(
        rx: Framed<tokio::io::ReadHalf<S>>,
        tx: Framed<tokio::io::WriteHalf<S>>,
    ) -> Self {
        Framed {
            inner: rx.inner.unsplit(tx.inner),
        }
    }
}

macro_rules! read_write_u {
    ($read_name:ident, $write_name:ident, $ty:ident, $len:expr) => {
        impl<S: AsyncWrite + Unpin> Framed<S> {
            pub async fn $write_name(
                &mut self,
                u: $ty,
            ) -> Result<(), FramedError> {
                self.inner.write_all(&u.to_le_bytes()).await?;
                Ok(())
            }
        }

        impl<S: AsyncRead + Unpin> Framed<S> {
            pub async fn $read_name(&mut self) -> Result<$ty, FramedError> {
                let mut buf = [0u8; $len];
                self.inner.read_exact(&mut buf).await?;
                Ok($ty::from_le_bytes(buf))
            }
        }
    };
}

read_write_u!(read_u8, write_u8, u8, 1);
read_write_u!(read_u16, write_u16, u16, 2);
read_write_u!(read_u32, write_u32, u32, 4);
read_write_u!(read_u64, write_u64, u64, 8);
read_write_u!(read_u128, write_u128, u128, 16);

impl<S: AsyncWrite + Unpin> Framed<S> {
    pub async fn write_ser<T>(
        &mut self,
        t: &T,
    ) -> Result<(), FramedError>
    where
        T: Ser,
    {
        let msg = kson::to_vec(t);
        self.write_u32(msg.len() as u32).await?;
        self.inner.write_all(&msg).await?;
        Ok(())
    }
}

impl<S: AsyncRead + Unpin> Framed<S> {
    pub async fn read_de<T>(&mut self) -> Result<T, FramedError>
    where
        T: De,
    {
        let len = self.read_u32().await?;
        let mut buf = vec![0u8; len as usize];
        self.inner.read_exact(&mut buf).await?;
        let res = kson::from_bytes(buf.into())?;
        Ok(res)
    }
}
