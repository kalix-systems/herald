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
    ) -> Result<Vec<sig::PublicKey>, Self::Error> {
        Ok(self
            .get_sigchain(of)?
            .map(|s| s.active_keys().into_iter().collect())
            .unwrap_or_default())
    }

    fn key_is_valid(
        &mut self,
        key: sig::PublicKey,
        valid_for: UserId,
    ) -> Result<bool, Self::Error>;

    fn all_active_keys(&mut self) -> Result<Vec<sig::PublicKey>, Self::Error>;
}

pub trait PendingStore: StoreLike {
    fn add_pending_payload(
        &mut self,
        id: PayloadId,
        payload: Payload,
        to: &[sig::PublicKey],
    ) -> Result<(), Self::Error>;

    fn get_pending_payload(
        &mut self,
        id: PayloadId,
    ) -> Result<Option<Payload>, Self::Error>;

    fn del_pending(
        &mut self,
        id: PayloadId,
        to: sig::PublicKey,
    ) -> Result<(), Self::Error>;
}
