use super::*;

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A new, signed key.
pub struct NewKey(pub Signed<sig::PublicKey>);

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
/// A key that is to be marked as deprecated.
pub struct DepKey(pub Signed<sig::PublicKey>);
