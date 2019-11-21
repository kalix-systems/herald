use super::*;
use crc32fast::Hasher;

pub const PACKET_SIZE: usize = 1024;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    pub content: Bytes,
    pub checksum: u32,
}

fn hash(buf: &[u8]) -> u32 {
    let mut h = Hasher::new();
    h.update(buf);
    h.finalize()
}

fn div_round_up(
    num: usize,
    den: usize,
) -> usize {
    (num + den - 1) / den
}

impl Packet {
    pub fn from_bytes(mut byt: Bytes) -> Vec<Self> {
        let num_packets = div_round_up(byt.len(), PACKET_SIZE);

        let mut out = Vec::with_capacity(num_packets);

        for _ in 0..num_packets - 1 {
            let content = byt.split_to(PACKET_SIZE);
            let checksum = hash(&content);
            out.push(Packet { content, checksum });
        }

        let checksum = hash(&byt);
        out.push(Packet {
            content: byt,
            checksum,
        });

        out
    }

    pub fn validate(&self) -> bool {
        self.content.len() <= PACKET_SIZE && hash(&self.content) == self.checksum
    }

    pub fn collect(packets: &[Self]) -> Option<Vec<u8>> {
        if packets.iter().all(Packet::validate) {
            let mut out = Vec::with_capacity(packets.iter().map(|p| p.content.len()).sum());
            for packet in packets {
                out.extend_from_slice(&packet.content);
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
