use super::*;
pub trait RatchetStore: StoreLike {
    fn get_ratchet(
        &mut self,
        with: sig::PublicKey,
    ) -> Result<Option<dr::DoubleRatchet>, Self::Error>;

    fn store_ratchet(
        &mut self,
        with: sig::PublicKey,
        ratchet: dr::DoubleRatchet,
    ) -> Result<(), Self::Error>;
}

pub trait SigStore: StoreLike {
    fn start_sigchain(
        &mut self,
        init: Signed<UserId>,
    ) -> Result<(), Self::Error>;

    fn extend_sigchain(
        &mut self,
        from: UserId,
        update: Signed<sig::SigUpdate>,
    ) -> Result<(), Self::Error>;

    fn get_sigchain(
        &mut self,
        of: UserId,
    ) -> Result<Option<sig::SigChain>, Self::Error>;

    fn active_keys(
        &mut self,
        of: UserId,
    ) -> Result<Vec<sig::PublicKey>, Self::Error>;

    fn key_is_valid(
        &mut self,
        key: sig::PublicKey,
        valid_for: UserId,
    ) -> Result<bool, Self::Error>;
}

pub trait ConversationStore: StoreLike {
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

    fn member_of(
        &mut self,
        cid: ConversationId,
        uid: UserId,
    ) -> Result<bool, Self::Error>;
}
