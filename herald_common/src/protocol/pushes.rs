use super::*;

#[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PushTag {
    Group,
    User,
    Device,
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub struct Push {
    pub tag: PushTag,
    pub timestamp: Time,
    pub msg: Bytes,
    pub gid: GlobalId,
}

#[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PushMeta {
    pub tag: PushTag,
    pub timestamp: Time,
}

pub mod catchup {
    use super::*;

    pub const CHUNK_SIZE: u32 = 256;

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub enum Catchup {
        Messages(Vec<Push>),
        Done,
    }

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub struct CatchupAck(pub u64);
}
