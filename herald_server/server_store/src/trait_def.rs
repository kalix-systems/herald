use super::*;

#[async_trait]
pub trait ServerStore {
    async fn get_sigchain(
        &mut self,
        user: UserId,
    ) -> Result<Option<sig::SigChain>, Error>;

    async fn recip_exists(
        &mut self,
        user: Recip,
    ) -> Result<bool, Error>;

    async fn add_to_sigchain(
        &mut self,
        new: Signed<sig::SigUpdate>,
    ) -> Result<PKIResponse, Error>;

    async fn user_of(
        &mut self,
        key: sig::PublicKey,
    ) -> Result<Option<UserId>, Error>;

    async fn new_prekeys<Keys: Stream<Item = (Signed<Prekey>, Option<Prekey>)> + Send>(
        &mut self,
        keys: Keys,
    ) -> Result<new_prekeys::Res, Error>;

    async fn get_random_prekeys<Keys: Stream<Item = sig::PublicKey> + Send>(
        &mut self,
        keys: Keys,
    ) -> Result<Vec<(sig::PublicKey, Signed<Prekey>)>, Error>;

    async fn add_to_group<Users: Stream<Item = UserId> + TryStreamExt + Send + Unpin>(
        &mut self,
        users: Users,
        conv: ConversationId,
    ) -> Result<add_to_group::Res, Error>;

    async fn leave_group<Convs: Stream<Item = ConversationId> + Send>(
        &mut self,
        user: UserId,
        groups: Convs,
    ) -> Result<leave_groups::Res, Error>;

    // should be done transactionally, returns Missing(r) for the first missing recip r
    // only adds to pending when it finds all devices
    async fn add_to_pending_and_get_valid_devs(
        &mut self,
        recip: &Recip,
        msg: &Push,
    ) -> Result<PushedTo, Error>;

    async fn get_pending(
        &mut self,
        of: sig::PublicKey,
    ) -> Result<Vec<(Push, i64)>, Error>;

    async fn del_pending<S: Stream<Item = i64> + Send>(
        &mut self,
        of: sig::PublicKey,
        items: S,
    ) -> Result<(), Error>;

    async fn new_user(
        &mut self,
        init: Signed<UserId>,
    ) -> Result<register::Res, Error>;
}
