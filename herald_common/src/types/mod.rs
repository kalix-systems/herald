use super::*;
use arrayvec::ArrayString;
use bytes::Bytes;
use kcl::random::*;
use kson::*;
use std::convert::TryFrom;

mod requests;
pub use requests::*;

mod pushes;
pub use pushes::*;

type UserIdInner = [u8; 32];

#[derive(Ser, De, Hash, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct UserId(ArrayString<UserIdInner>);

impl std::ops::Deref for UserId {
    type Target = ArrayString<UserIdInner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UserId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug)]
pub enum InvalidUserId {
    NonAlphaNumeric,
    CapacityError,
}

impl std::fmt::Display for UserId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl std::fmt::Display for InvalidUserId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
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
            Err(InvalidUserId::NonAlphaNumeric)
        } else {
            Ok(Self(
                ArrayString::from(val).map_err(|_| InvalidUserId::CapacityError)?,
            ))
        }
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy, Ser, De)]
/// Conversation ID
pub struct ConversationId(pub UQ);

impl ConversationId {
    /// Creates a new random `ConversationId`
    pub fn gen_new() -> Self {
        Self(UQ::gen_new())
    }

    /// Returns byte slice from `ConversationId`
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub struct UserMeta {
    pub initial: Signed<UserId>,
    pub sig_chain: Vec<Signed<sig::SigUpdate>>,
}

#[derive(Ser, De, Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlobalId {
    pub uid: UserId,
    pub did: sig::PublicKey,
}

impl std::convert::AsRef<sig::PublicKey> for GlobalId {
    fn as_ref(&self) -> &sig::PublicKey {
        &self.did
    }
}

#[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PKIResponse {
    Success,
    BadSig(SigValid),
    Redundant,
    DeadKey,
}
