use crate::*;

pub trait Store {
    type Error;

    fn add_key(&mut self, uid: UserId, key: Signed<sig::PublicKey>) -> Result<bool, Self::Error>;
    fn read_key(&mut self, uid: UserId, key: sig::PublicKey) -> Result<sig::PKMeta, Self::Error>;
    fn deprecate_key(
        &mut self,
        uid: UserId,
        key: Signed<sig::PublicKey>,
    ) -> Result<bool, Self::Error>;

    fn user_exists(&mut self, uid: UserId) -> Result<bool, Self::Error>;
    fn key_is_valid(&mut self, uid: UserId, key: sig::PublicKey) -> Result<bool, Self::Error>;
    fn read_meta(&mut self, uid: UserId) -> Result<UserMeta, Self::Error>;

    fn add_prekey(&mut self, pre: sealed::PublicKey) -> Result<bool, Self::Error>;
    fn get_prekey(&mut self, key: sig::PublicKey) -> Result<sealed::PublicKey, Self::Error>;

    fn add_pending(&mut self, key: sig::PublicKey, msg: Push) -> Result<(), Self::Error>;
    fn get_pending(&mut self, key: sig::PublicKey) -> Result<Vec<Push>, Self::Error>;
    fn remove_pending(&mut self, key: sig::PublicKey) -> Result<(), Self::Error>;
}
