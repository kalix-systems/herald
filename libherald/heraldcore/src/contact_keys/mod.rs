use crate::errors::HErr;
use herald_common::{sign, Signed};

pub(crate) fn key_deprecated(k: Signed<sign::PublicKey>) -> Result<(), HErr> {
    Ok(())
}

pub(crate) fn key_registered(k: Signed<sign::PublicKey>) -> Result<(), HErr> {
    Ok(())
}

#[cfg(test)]
mod tests;
