use kson::prelude::*;
use std::time::Duration;
use tokio::{prelude::*, time::*};

mod packets;
use packets::*;

#[derive(Debug)]
pub enum FramedError {
    IO(std::io::Error),
    Encoding(KsonError),
    TimedOut(tokio::time::Elapsed),
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
from_fn!(FramedError, KsonError, FramedError::Encoding);
from_fn!(FramedError, tokio::time::Elapsed, FramedError::TimedOut);

pub struct Framed<S> {
    inner: S,
    dur: Duration,
    packet_size: usize,
}

impl<S> Framed<S> {
    pub fn new(
        inner: S,
        dur: Duration,
        packet_size: usize,
    ) -> Self {
        Framed {
            inner,
            dur,
            packet_size,
        }
    }
}

impl<S: AsyncWrite + Unpin> Framed<S> {
    async fn write_u64(
        &mut self,
        u: u64,
    ) -> Result<(), FramedError> {
        self.inner.write_all(&u.to_le_bytes()).await?;
        Ok(())
    }

    async fn write_u64_timed(
        &mut self,
        u: u64,
    ) -> Result<(), FramedError> {
        timeout(self.dur, self.write_u64(u)).await?
    }

    pub async fn write<T>(
        &mut self,
        t: &T,
    ) -> Result<(), FramedError>
    where
        T: Ser,
    {
        let msg = kson::to_vec(t);
        self.write_u64(msg.len() as u64).await?;
        self.inner.write_all(&msg).await?;
        Ok(())
    }

    pub async fn write_timed<T>(
        &mut self,
        t: &T,
    ) -> Result<(), FramedError>
    where
        T: Ser,
    {
        timeout(self.dur, self.write(t)).await?
    }
}

impl<S: AsyncRead + Unpin> Framed<S> {
    async fn read_u64(&mut self) -> Result<u64, FramedError> {
        let mut buf = [0u8; 8];
        self.inner.read_exact(&mut buf).await?;
        Ok(u64::from_le_bytes(buf))
    }

    async fn read_u64_timed(&mut self) -> Result<u64, FramedError> {
        timeout(self.dur, self.read_u64()).await?
    }

    pub async fn read<T>(&mut self) -> Result<T, FramedError>
    where
        T: De,
    {
        let len = self.read_u64().await?;
        let mut buf = vec![0u8; len as usize];
        self.inner.read_exact(&mut buf).await?;
        let res = kson::from_bytes(buf.into())?;
        Ok(res)
    }

    pub async fn read_timed<T>(&mut self) -> Result<T, FramedError>
    where
        T: De,
    {
        timeout(self.dur, self.read()).await?
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> Framed<S> {
    pub async fn read_packeted<T>(&mut self) -> Result<T, FramedError>
    where
        T: De,
    {
        let len: usize;

        loop {
            let maybe_len = self.read_u64_timed().await?;

            self.write_u64_timed(maybe_len).await?;

            match timeout(self.dur, self.read()).await?? {
                PacketResponse::Success => {
                    len = maybe_len as usize;
                    break;
                }
                PacketResponse::Retry => {}
            }
        }

        let mut packets = Vec::with_capacity(len);
        let collected: Vec<u8>;

        loop {
            packets.clear();

            for _ in 0..len {
                let packet = self.read_timed().await?;
                packets.push(packet);
            }

            if let Some(bytes) = Packet::collect(&packets) {
                self.write_timed(&PacketResponse::Success).await?;
                collected = bytes;
                break;
            } else {
                self.write_timed(&PacketResponse::Retry).await?;
            }
        }

        Ok(kson::from_bytes(collected.into())?)
    }

    pub async fn write_packeted<T>(
        &mut self,
        t: &T,
    ) -> Result<(), FramedError>
    where
        T: Ser,
    {
        let bvec = Bytes::from(kson::to_vec(t));
        let packets = Packet::from_bytes(self.packet_size, bvec);
        let len = packets.len() as u64;

        loop {
            self.write_u64_timed(len).await?;

            if len == self.read_u64_timed().await? {
                self.write_timed(&PacketResponse::Success).await?;
                break;
            } else {
                self.write_timed(&PacketResponse::Retry).await?;
            }
        }

        loop {
            for packet in packets.iter() {
                timeout(self.dur, self.write(packet)).await??;
            }

            match timeout(self.dur, self.read()).await?? {
                PacketResponse::Success => break,
                PacketResponse::Retry => {}
            }
        }

        Ok(())
    }
}
