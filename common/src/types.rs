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

pub mod push {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct PushReq {
        pub to_users: Vec<UserId>,
        pub to_devs: Vec<sig::PublicKey>,
        pub msg: Push,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum Response {
        Success,
        Missing(Vec<UserId>, Vec<sig::PublicKey>),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tagged<T> {
    pub mid: UQ,
    pub dat: T,
}

impl<T> Tagged<T> {
    pub fn new(t: T) -> Self {
        Tagged {
            mid: UQ::new(),
            dat: t,
        }
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy)]
#[repr(u8)]
pub enum SessionType {
    Register = 0,
    Login = 1,
}

impl TryFrom<u8> for SessionType {
    type Error = u8;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Register),
            1 => Ok(Self::Login),
            i => Err(i),
        }
    }
}

impl Serialize for SessionType {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for SessionType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::*;
        let u = u8::deserialize(d)?;
        u.try_into().map_err(|u| {
            Error::invalid_value(
                Unexpected::Unsigned(u64::from(u)),
                &format!("expected a value between {} and {}", 0, 2).as_str(),
            )
        })
    }
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
    pub enum ToServer {
        RequestUID(UserId),
        UseKey(Signed<sign::PublicKey>),
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ToClient {
        UIDTaken,
        UIDReady,
        KeyTaken,
        BadSig,
        KeyReady,
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
