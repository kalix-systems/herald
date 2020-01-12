use crate::ratchet::double as dr;
use herald_common::*;
use kcl::*;
use std::error::Error;

#[derive(Ser, De, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub struct PayloadId(random::UQ);

#[derive(Ser, De, Eq, PartialEq, Hash, Clone)]
pub enum Payload {
    SigUpdate(Signed<sig::SigUpdate>),
    NewConvo(ConversationId, Vec<UserId>),
    AddToConvo(ConversationId, Vec<UserId>),
    LeaveConvo(ConversationId),
    Msg(Bytes),
}

#[derive(Ser, De, Eq, PartialEq, Hash, Clone)]
pub enum Msg<E: Error + Send + 'static> {
    Encrypted {
        id: PayloadId,
        header: dr::Header,
        payload: Bytes,
    },
    Success(PayloadId),
    Failed(PayloadId, dr::DecryptError<E>),
}
