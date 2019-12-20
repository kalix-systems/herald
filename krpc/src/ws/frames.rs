use super::*;
use byteorder::*;

#[derive(Debug)]
pub(super) enum ServerFrame<Res, Push> {
    Res(u64, Res),
    Psh(u64, Push),
}

use ServerFrame::*;

impl<Res: Ser, Push: Ser> ServerFrame<Res, Push> {
    pub(super) fn to_vec(&self) -> Vec<u8> {
        let mut start = Vec::with_capacity(9);
        match self {
            Res(u, r) => {
                start.push(0);
                start.extend_from_slice(&u.to_le_bytes());
                let mut ser = Serializer(start);
                r.ser(&mut ser);
                ser.0
            }
            Psh(u, p) => {
                start.push(1);
                start.extend_from_slice(&u.to_le_bytes());
                let mut ser = Serializer(start);
                p.ser(&mut ser);
                ser.0
            }
        }
    }
}

impl<Res: De, Push: De> ServerFrame<Res, Push> {
    pub(super) fn from_bytes(bytes: Bytes) -> Result<Self, Error> {
        if bytes.len() <= 10 {
            bail!("server frame too short")
        }
        let tag = bytes[0];
        let u = LittleEndian::read_u64(&bytes[1..9]);
        let rest = bytes.slice(9..);
        drop(bytes);
        match tag {
            0 => {
                let res =
                    kson::from_bytes(rest).context("failed to deserialize response content")?;
                Ok(Res(u, res))
            }
            1 => {
                let psh = kson::from_bytes(rest).context("failed to deserialize push content")?;
                Ok(Psh(u, psh))
            }
            t => Err(anyhow!("unknown tag {:x}", t)),
        }
    }
}

#[derive(Debug)]
pub(super) enum ClientFrame<Req, PushAck> {
    Req(u64, Req),
    Ack(u64, PushAck),
}

use ClientFrame::*;

impl<Req: Ser, PushAck: Ser> ClientFrame<Req, PushAck> {
    pub(super) fn to_vec(&self) -> Vec<u8> {
        let mut start = Vec::with_capacity(9);
        match self {
            Req(u, r) => {
                start.push(0);
                start.extend_from_slice(&u.to_le_bytes());
                let mut ser = Serializer(start);
                r.ser(&mut ser);
                ser.0
            }
            Ack(u, a) => {
                start.push(1);
                start.extend_from_slice(&u.to_le_bytes());
                let mut ser = Serializer(start);
                a.ser(&mut ser);
                ser.0
            }
        }
    }
}

impl<Req: De, PushAck: De> ClientFrame<Req, PushAck> {
    pub(super) fn from_bytes(bytes: Bytes) -> Result<Self, Error> {
        if bytes.len() <= 9 {
            bail!("client frame too short")
        }
        let tag = bytes[0];
        let u = LittleEndian::read_u64(&bytes[1..9]);
        let rest = bytes.slice(9..);
        drop(bytes);
        match tag {
            0 => {
                let req =
                    kson::from_bytes(rest).context("failed to deserialize request content")?;
                Ok(Req(u, req))
            }
            1 => {
                let ack = kson::from_bytes(rest).context("failed to deserialize ack content")?;
                Ok(Ack(u, ack))
            }
            t => Err(anyhow!("unknown tag {:x}", t)),
        }
    }
}
