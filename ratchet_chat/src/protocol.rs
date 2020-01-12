use super::*;
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
    Failed {
        id: PayloadId,
        reason: Option<dr::DecryptError<E>>,
    },
}

pub trait SigStore: StoreLike {
    fn start_sigchain(
        &mut self,
        init: Signed<UserId>,
    ) -> Result<(), Self::Error>;

    fn extend_sigchain(
        &mut self,
        from: sig::PublicKey,
        update: Signed<sig::SigUpdate>,
    ) -> Result<(), Self::Error>;

    fn get_sigchain(
        &mut self,
        of: UserId,
    ) -> Result<Option<sig::SigChain>, Self::Error>;
}

pub trait ConversationStore: StoreLike {
    fn new_convo(
        &mut self,
        cid: ConversationId,
        members: Vec<UserId>,
    ) -> Result<(), Self::Error>;

    fn add_to_convo(
        &mut self,
        cid: ConversationId,
        members: Vec<UserId>,
    ) -> Result<(), Self::Error>;

    fn left_convo(
        &mut self,
        cid: ConversationId,
        from: UserId,
    ) -> Result<(), Self::Error>;

    fn get_members(
        &mut self,
        cid: ConversationId,
    ) -> Result<Option<Vec<UserId>>, Self::Error>;
}

pub trait RatchetStore: StoreLike {
    fn get_rachet(
        &mut self,
        with: sig::PublicKey,
    ) -> Result<Option<dr::DoubleRatchet>, Self::Error>;

    fn store_ratchet(
        &mut self,
        with: sig::PublicKey,
        ratchet: dr::DoubleRatchet,
    ) -> Result<(), Self::Error>;
}
