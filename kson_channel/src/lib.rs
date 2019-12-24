use kson::prelude::*;
// use std::time::Duration;
use thiserror::Error;
// use tokio::{prelude::*, time::*};
use tokio::prelude::*;

// mod packets;
// use packets::*;

#[derive(Debug, Error)]
pub enum FramedError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Deserialization: {0}")]
    Encoding(#[from] KsonError),
    #[error("Timed out: {0}")]
    TimedOut(#[from] tokio::time::Elapsed),
}

pub struct Framed<S> {
    inner: S,
    // dur: Duration,
    // packet_size: usize,
}

impl<S> Framed<S> {
    pub fn new(
        inner: S,
        // dur: Duration,
        // packet_size: usize,
    ) -> Self {
        Framed {
            inner,
            // dur,
            // packet_size,
        }
    }

    pub fn into_inner(self) -> S {
        self.inner
    }
}

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

impl<S: AsyncWrite + Unpin> Framed<S> {
    async fn write_u32(
        &mut self,
        u: u32,
    ) -> Result<(), FramedError> {
        self.inner.write_all(&u.to_le_bytes()).await?;
        Ok(())
    }

    // async fn write_u32_timed(
    //     &mut self,
    //     u: u32,
    // ) -> Result<(), FramedError> {
    //     timeout(self.dur, self.write_u32(u)).await?
    // }

    pub async fn write<T>(
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

    // pub async fn write_timed<T>(
    //     &mut self,
    //     t: &T,
    // ) -> Result<(), FramedError>
    // where
    //     T: Ser,
    // {
    //     timeout(self.dur, self.write(t)).await?
    // }
}

impl<S: AsyncRead + Unpin> Framed<S> {
    async fn read_u32(&mut self) -> Result<u32, FramedError> {
        let mut buf = [0u8; 4];
        self.inner.read_exact(&mut buf).await?;
        Ok(u32::from_le_bytes(buf))
    }

    // async fn read_u32_timed(&mut self) -> Result<u32, FramedError> {
    //     timeout(self.dur, self.read_u32()).await?
    // }

    pub async fn read<T>(&mut self) -> Result<T, FramedError>
    where
        T: De,
    {
        let len = self.read_u32().await?;
        let mut buf = vec![0u8; len as usize];
        self.inner.read_exact(&mut buf).await?;
        let res = kson::from_bytes(buf.into())?;
        Ok(res)
    }

    // pub async fn read_timed<T>(&mut self) -> Result<T, FramedError>
    // where
    //     T: De,
    // {
    //     timeout(self.dur, self.read()).await?
    // }
}

// impl<S: AsyncRead + AsyncWrite + Unpin> Framed<S> {
//     pub async fn read_packeted<T>(&mut self) -> Result<T, FramedError>
//     where
//         T: De,
//     {
//         let len: usize;

//         loop {
//             let maybe_len = self.read_u32_timed().await?;

//             self.write_u32_timed(maybe_len).await?;

//             match timeout(self.dur, self.read()).await?? {
//                 PacketResponse::Success => {
//                     len = maybe_len as usize;
//                     break;
//                 }
//                 PacketResponse::Retry => {}
//             }
//         }

//         let mut packets = Vec::with_capacity(len);
//         let collected: Vec<u8>;

//         loop {
//             packets.clear();

//             for _ in 0..len {
//                 let packet = self.read_timed().await?;
//                 packets.push(packet);
//             }

//             if let Some(bytes) = Packet::collect(&packets) {
//                 self.write_timed(&PacketResponse::Success).await?;
//                 collected = bytes;
//                 break;
//             } else {
//                 self.write_timed(&PacketResponse::Retry).await?;
//             }
//         }

//         Ok(kson::from_bytes(collected.into())?)
//     }

//     pub async fn write_packeted<T>(
//         &mut self,
//         t: &T,
//     ) -> Result<(), FramedError>
//     where
//         T: Ser,
//     {
//         let bvec = Bytes::from(kson::to_vec(t));
//         let packets = Packet::from_bytes(self.packet_size, bvec);
//         let len = packets.len() as u32;

//         loop {
//             self.write_u32_timed(len).await?;

//             if len == self.read_u32_timed().await? {
//                 self.write_timed(&PacketResponse::Success).await?;
//                 break;
//             } else {
//                 self.write_timed(&PacketResponse::Retry).await?;
//             }
//         }

//         loop {
//             for packet in packets.iter() {
//                 timeout(self.dur, self.write(packet)).await??;
//             }

//             match timeout(self.dur, self.read()).await?? {
//                 PacketResponse::Success => break,
//                 PacketResponse::Retry => {}
//             }
//         }

//         Ok(())
//     }
// }
