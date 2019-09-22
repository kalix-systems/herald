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

pub mod fanout {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum ToServer {
        UID {
            to: Vec<UserId>,
            msg: Bytes,
        },
        DID {
            to: Vec<sign::PublicKey>,
            msg: Bytes,
        },
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum ServerResponse {
        Success,
        MissingUIDs(Vec<UserId>),
        MissingDIDs(Vec<sign::PublicKey>),
    }
}

pub mod pubkey {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ToServer {
        RegisterKey(Signed<sign::PublicKey>),
        DeprecateKey(Signed<sign::PublicKey>),
        RegisterPrekey(sealed::PublicKey),
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ServerResponse {
        Success,
        BadSignature,
        Redundant,
        DeadKey,
    }
}

pub mod query {
    use super::*;

    // TODO: consider making these batchable?
    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ToServer {
        UserExists(UserId),
        UserKeys(UserId),
        GetPrekey(sign::PublicKey),
        KeyMeta(UserId, sign::PublicKey),
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum ServerResponse {
        Exists(bool),
        Keys(UserMeta),
        KeyMeta(sig::PKMeta),
        PreKey(Signed<box_::PublicKey>),
        MissingData,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Push {
    KeyRegistered(Signed<sign::PublicKey>),
    KeyDeprecated(Signed<sign::PublicKey>),
    NewUMessage { from: GlobalId, msg: Bytes },
    NewDMessage { from: GlobalId, msg: Bytes },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Fanout(fanout::ServerResponse),
    PKI(pubkey::ServerResponse),
    Query(query::ServerResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum MessageToClient {
    Push(Push),
    Response(Response),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum MessageToServer {
    Fanout(fanout::ToServer),
    PKI(pubkey::ToServer),
    Query(query::ToServer),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tagged<T> {
    pub mid: [u8; 32],
    pub dat: T,
}

impl<T> Tagged<T> {
    pub fn new(t: T) -> Self {
        Tagged {
            mid: rand_id(),
            dat: t,
        }
    }
}

fn rand_id() -> [u8; 32] {
    use sodiumoxide::randombytes::randombytes_into;
    sodiumoxide::init().expect("failed to init libsodium - what have you done");
    let mut buf = [0u8; 32];
    randombytes_into(&mut buf);
    buf
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
