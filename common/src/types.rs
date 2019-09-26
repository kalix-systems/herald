use crate::crypto::*;
use arrayvec::ArrayString;
use bytes::Bytes;
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

type UserIdInner = [u8; 32];

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq, Copy)]
pub struct UserId(ArrayString<UserIdInner>);

impl std::ops::Deref for UserId {
    type Target = ArrayString<UserIdInner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub enum InvalidUserId {
    NonAlphaNumeric,
    CapacityError,
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl std::fmt::Display for InvalidUserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use InvalidUserId::*;
        match self {
            NonAlphaNumeric => write!(f, "InvalidUserId: non-alphanumeric characters not allowed"),
            CapacityError => write!(f, "InvalidUserId: UserId must be 32 bytes or less"),
        }
    }
}

impl std::error::Error for InvalidUserId {}

impl TryFrom<&str> for UserId {
    type Error = InvalidUserId;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        if !val.bytes().all(|c| c.is_ascii_alphanumeric()) {
            return Err(InvalidUserId::NonAlphaNumeric);
        } else {
            Ok(Self(
                ArrayString::from(val).map_err(|_| InvalidUserId::CapacityError)?,
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserMeta {
    pub keys: HashMap<sig::PublicKey, sig::PKMeta>,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlobalId {
    pub uid: UserId,
    pub did: sig::PublicKey,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PKIResponse {
    Success,
    BadSignature,
    Redundant,
    DeadKey,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Push {
    KeyRegistered(Signed<sign::PublicKey>),
    KeyDeprecated(Signed<sign::PublicKey>),
    NewUMessage {
        timestamp: DateTime<Utc>,
        from: GlobalId,
        msg: Bytes,
    },
    NewDMessage {
        timestamp: DateTime<Utc>,
        from: GlobalId,
        msg: Bytes,
    },
}

pub mod keys_of {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub Vec<UserId>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Res(pub HashMap<UserId, UserMeta>);
}

pub mod key_info {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub Vec<sig::PublicKey>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Res(pub HashMap<sig::PublicKey, sig::PKMeta>);
}

pub mod keys_exist {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub Vec<sig::PublicKey>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Res(pub Vec<bool>);
}

pub mod users_exist {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub Vec<UserId>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Res(pub Vec<bool>);
}

pub mod push {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req {
        pub to_users: Vec<UserId>,
        pub to_devs: Vec<sig::PublicKey>,
        pub msg: Push,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum Res {
        Success,
        Missing(Vec<UserId>, Vec<sig::PublicKey>),
    }
}

pub mod new_key {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub Signed<sig::PublicKey>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Res(pub PKIResponse);
}

pub mod dep_key {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub Signed<sig::PublicKey>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Res(pub PKIResponse);
}

pub mod login {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SignAs(pub GlobalId);

    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SignAsResponse {
        Sign(UQ),
        KeyDeprecated,
        MissingUID,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LoginToken(pub sign::Signature);

    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum LoginTokenResponse {
        Success,
        BadSig,
    }
}

pub mod register {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Req(pub UserId, pub Signed<sign::PublicKey>);

    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Res {
        UIDTaken,
        KeyTaken,
        BadSig,
        Success,
    }
}

pub mod catchup {
    use super::*;

    pub const CHUNK_SIZE: usize = 256;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Catchup(pub Vec<Push>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct CatchupAck(pub u64);
}
