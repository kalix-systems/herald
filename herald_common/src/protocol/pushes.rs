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
}

#[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PushMeta {
    pub tag: PushTag,
    pub timestamp: Time,
}

pub mod register {
    use super::*;

    pub type Claim = Signed<UserId>;

    #[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Res {
        Success,
        UserAlreadyClaimed,
        BadSig,
    }
}
pub mod login {
    use super::*;

    #[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SignAs(pub GlobalId);

    #[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SignAsResponse {
        Sign(UQ),
        SessionExists,
        KeyDeprecated,
        MissingUID,
    }

    #[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LoginToken(pub sig::Signature);

    #[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum LoginTokenResponse {
        Success,
        BadSig,
    }
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
