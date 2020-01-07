use kcl::*;
use kson::*;

#[derive(Ser, De, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RatchetId(random::UQ);

pub trait Conversations {
    fn add_key(
        &mut self,
        id: RatchetId,
        key: sign::PublicKey,
    );
}
