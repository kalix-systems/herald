use std::{error, fmt};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ChainError {
    CryptoError,
    DecryptionError,
    BadSig,
    MissingKeys,
    BlockStoreUnavailable,
    RedundantMark,
    BlockStoreCorrupted,
}

use ChainError::*;

impl fmt::Display for ChainError {
    fn fmt(
        &self,
        fmt: &mut fmt::Formatter,
    ) -> Result<(), fmt::Error> {
        match self {
            CryptoError => write!(fmt, "libsodium failed"),
            DecryptionError => write!(fmt, "Failed to decrypt a block"),
            BadSig => write!(fmt, "Signature did not sign data"),
            MissingKeys => write!(fmt, "BlockStore couldn't find keys"),
            BlockStoreUnavailable => write!(fmt, "BlockStore is unavailable"),
            BlockStoreCorrupted => write!(fmt, "BlockStore is possibly corrupted"),
            RedundantMark => write!(
                fmt,
                "Tried to mark key as used that was already marked as used."
            ),
        }
    }
}

impl error::Error for ChainError {}
