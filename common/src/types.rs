use crate::crypto::*;
use arrayvec::ArrayString;
use serde::*;
use sodiumoxide::crypto::{box_, sign};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq, Copy)]
pub struct UserId(ArrayString<[u8; 32]>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserMeta {
    keys: HashMap<sig::PublicKey, sig::PKMeta>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GlobalId {
    pub uid: UserId,
    pub did: sig::PublicKey,
}

pub mod fanout {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum ToServer<'a> {
        UID {
            to: Vec<UserId>,
            msg: &'a [u8],
        },
        DID {
            to: Vec<sign::PublicKey>,
            msg: &'a [u8],
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

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum ToServer {
        RegisterKey(Signed<sign::PublicKey>),
        DeprecateKey(Signed<sign::PublicKey>),
        RegisterPrekey(Signed<box_::PublicKey>),
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

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum ToServer {
        UserExists(UserId),
        UserKeys(UserId),
        GetPrekey(sign::PublicKey),
        KeyMeta(UserId, sign::PublicKey),
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ServerResponse {
        Exists(bool),
        // Keys(UserMeta),
        // KeyMeta(sig::PKMeta),
        // PreKey(Signed<box_::PublicKey>),
        MissingData,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Push<'a> {
    KeyRegistered(Signed<sign::PublicKey>),
    KeyDeprecated(Signed<sign::PublicKey>),
    NewUMessage { from: GlobalId, msg: &'a [u8] },
    NewDMessage { from: GlobalId, msg: &'a [u8] },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Fanout(fanout::ServerResponse),
    PKI(pubkey::ServerResponse),
    Query(query::ServerResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum MessageToClient<'a> {
    Push(#[serde(borrow)] Push<'a>),
    Response(Response),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum MessageToServer<'a> {
    Fanout(#[serde(borrow)] fanout::ToServer<'a>),
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
