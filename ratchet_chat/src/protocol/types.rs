use herald_common::*;
use kcl::*;

#[derive(Ser, De, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RatchetId(random::UQ);

pub trait Conversations {
    type Error: std::error::Error + Send + 'static;

    fn add_key(
        &mut self,
        id: RatchetId,
        key: sign::PublicKey,
    );
}

#[derive(Ser, De)]
pub enum Msg {
    SigUpdate(Signed<sig::SigUpdate>),
    Msg(Bytes),
}
