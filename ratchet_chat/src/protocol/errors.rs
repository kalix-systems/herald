use super::*;

#[derive(Ser, De, Eq, PartialEq, Hash, Clone, Copy)]
pub enum FailureReason {
    Decryption,
    Deserialization,
    BadSig(SigValid),
    InvalidSender,
    StoreError,
    WhoKnows,
}

#[derive(Error, Debug)]
pub enum TransitError<E: StdError + Send + 'static> {
    #[error("Failed to decrypt: {0}")]
    Decryption(#[from] dr::DecryptError<E>),
    #[error("Failed to deserialize: {0}")]
    Kson(#[from] KsonError),
    #[error("Bare store error: {0}")]
    Store(E),
    #[error("Tried to retreive ratchet with {0:#?} but none were found")]
    NoSession(sig::PublicKey),
    #[error("Tried to encrypt using uninitialized ratchet with {0:#?}, THIS SHOULD NEVER HAPPEN")]
    Uninit(sig::PublicKey),
}

impl<E: StdError + Send + 'static> From<TransitError<E>> for FailureReason {
    fn from(e: TransitError<E>) -> Self {
        match e {
            TransitError::Decryption(de) => match de {
                dr::DecryptError::StoreError(_) => FailureReason::StoreError,
                _ => FailureReason::Decryption,
            },
            TransitError::Kson(_) => FailureReason::Deserialization,
            TransitError::Store(_) => FailureReason::StoreError,
            TransitError::NoSession(_) => FailureReason::WhoKnows,
            TransitError::Uninit(_) => FailureReason::WhoKnows,
        }
    }
}

#[derive(Debug, Error)]
pub enum PayloadError<E: StdError + Send + 'static> {
    #[error("Bare store error: {0}")]
    Store(E),
    #[error("Invalid signature: {0:#?}")]
    BadSig(SigValid),
    #[error("Message should not have been sent by this device, the sender is being sketchy")]
    InvalidSender,
}

impl<E: StdError + Send + 'static> From<PayloadError<E>> for FailureReason {
    fn from(e: PayloadError<E>) -> Self {
        match e {
            PayloadError::Store(_) => FailureReason::StoreError,
            PayloadError::BadSig(v) => FailureReason::BadSig(v),
            PayloadError::InvalidSender => FailureReason::InvalidSender,
        }
    }
}
