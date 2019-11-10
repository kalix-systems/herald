use super::*;
use std::time::Duration;
use tokio::prelude::*;

#[derive(Debug)]
pub enum FramedError {
    IO(std::io::Error),
    Encoding(serde_cbor::Error),
    TimedOut(tokio::timer::timeout::Elapsed),
}

macro_rules! from_fn {
    ($to:ty, $from:ty, $fn:expr) => {
        impl From<$from> for $to {
            fn from(f: $from) -> $to {
                $fn(f)
            }
        }
    };
}

from_fn!(FramedError, std::io::Error, FramedError::IO);
from_fn!(FramedError, serde_cbor::Error, FramedError::Encoding);
from_fn!(
    FramedError,
    tokio::timer::timeout::Elapsed,
    FramedError::TimedOut
);

pub struct Framed<S> {
    inner: S,
}

const TIMEOUT_DUR: Duration = Duration::from_secs(10);

impl<S> Framed<S> {
    pub fn new(inner: S) -> Self {
        Framed { inner }
    }
    pub async fn write<T>(&mut self, t: &T) -> Result<(), FramedError>
    where
        T: Serialize,
        S: AsyncWrite + Unpin,
    {
        let msg = serde_cbor::to_vec(t)?;
        let len_bytes = u64::to_le_bytes(msg.len() as u64);
        self.inner.write_all(&len_bytes).await?;
        self.inner.write_all(&msg).await?;
        Ok(())
    }

    pub async fn read<T>(&mut self) -> Result<T, FramedError>
    where
        S: AsyncRead + Unpin,
        T: serde::de::DeserializeOwned,
    {
        let mut buf = [0u8; 8];
        self.inner.read_exact(&mut buf).await?;
        let len = u64::from_le_bytes(buf);
        let mut buf = vec![0u8; len as usize];
        self.inner.read_exact(&mut buf).await?;
        let res = serde_cbor::from_slice(&buf)?;
        Ok(res)
    }

    pub async fn write_packeted<T>(&mut self, t: &T) -> Result<(), FramedError>
    where
        T: Serialize,
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let bvec = Bytes::from(serde_cbor::to_vec(t)?);
        let packets = Packet::from_bytes(bvec);
        let len = packets.len() as u64;

        loop {
            self.write(&ServerTransmission::Packets(len))
                .timeout(TIMEOUT_DUR)
                .await??;

            if len == self.read::<u64>().timeout(TIMEOUT_DUR).await?? {
                self.write(&PacketResponse::Success)
                    .timeout(TIMEOUT_DUR)
                    .await??;
                break;
            } else {
                self.write(&PacketResponse::Retry)
                    .timeout(TIMEOUT_DUR)
                    .await??;
            }
        }

        loop {
            for packet in packets.iter() {
                self.write(packet).timeout(TIMEOUT_DUR).await??;
            }

            match self.read().timeout(TIMEOUT_DUR).await?? {
                PacketResponse::Success => break,
                PacketResponse::Retry => {}
            }
        }

        Ok(())
    }
}
