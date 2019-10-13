use super::*;
use crc32fast::Hasher;

pub const PACKET_SIZE: usize = 1024;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Packet<'a> {
    #[serde(with = "serde_bytes")]
    pub content: &'a [u8],
    pub checksum: u32,
}

fn hash(buf: &[u8]) -> u32 {
    let mut h = Hasher::new();
    h.update(buf);
    h.finalize()
}

impl<'a> Packet<'a> {
    pub fn from_slice(buf: &'a [u8]) -> Vec<Self> {
        buf.chunks(PACKET_SIZE)
            .map(|content| {
                let checksum = hash(buf);
                Packet { content, checksum }
            })
            .collect()
    }

    pub fn validate(&self) -> bool {
        self.content.len() <= PACKET_SIZE && hash(self.content) == self.checksum
    }

    pub fn collect(packets: &[Self]) -> Option<Vec<u8>> {
        if packets.iter().all(Packet::validate) {
            let mut out = Vec::with_capacity(packets.iter().map(|p| p.content.len()).sum());
            for packet in packets {
                out.extend_from_slice(packet.content);
            }
            Some(out)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketResponse {
    Success,
    Retry,
}
