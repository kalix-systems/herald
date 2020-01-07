use super::*;
use double_ratchet::*;

pub struct KxKeyPair(pub kx::KeyPair);

impl KeyPair for KxKeyPair {
    type PublicKey = kx::PublicKey;
    fn new<R>(_: &mut R) -> Self {
        Self(kx::KeyPair::gen_new())
    }

    fn public(&self) -> &kx::PublicKey {
        &self.0.public
    }
}

pub struct DefaultProvider {}

impl CryptoProvider for DefaultProvider {
    type PublicKey = kx::PublicKey;
    type KeyPair = KxKeyPair;
    type SharedSecret = hash::Key;
    type RootKey = hash::Key;
    type ChainKey = hash::Key;
    type MessageKey = aead::Key;

    fn diffie_hellman(
        us: &Self::KeyPair,
        them: &Self::PublicKey,
    ) -> Self::SharedSecret {
        let mut hk_buf = [0u8; hash::KEY_LEN];
        us.0.symmetric_kx_into(*them, &mut hk_buf);
        hash::Key(hk_buf)
    }

    fn kdf_rk(
        root_key: &Self::RootKey,
        shared_secret: &Self::SharedSecret,
    ) -> (Self::RootKey, Self::ChainKey) {
        let mut rk_buf = [0u8; hash::KEY_LEN];
        let mut ck_buf = [0u8; hash::KEY_LEN];
        let mut bufs: [&mut [u8]; 2] = [&mut rk_buf, &mut ck_buf];
        root_key.hash_into_many(
            shared_secret.as_ref(),
            bufs.iter_mut().map(std::ops::DerefMut::deref_mut),
        );
        (hash::Key(rk_buf), hash::Key(ck_buf))
    }

    fn kdf_ck(chain_key: &Self::ChainKey) -> (Self::ChainKey, Self::MessageKey) {
        let mut ck_buf = [0u8; hash::KEY_LEN];
        let mut mk_buf = [0u8; aead::KEY_LEN];
        let mut bufs: [&mut [u8]; 2] = [&mut ck_buf, &mut mk_buf];
        chain_key.hash_into_many(&[], bufs.iter_mut().map(std::ops::DerefMut::deref_mut));
        (hash::Key(ck_buf), aead::Key(mk_buf))
    }

    fn encrypt(
        key: &Self::MessageKey,
        plaintext: &[u8],
        associated_data: &[u8],
    ) -> Vec<u8> {
        key.seal_attached(associated_data, plaintext)
    }

    fn decrypt(
        key: &Self::MessageKey,
        ciphertext: &[u8],
        associated_data: &[u8],
    ) -> Result<Vec<u8>, DecryptError> {
        key.open_attached(associated_data, ciphertext)
            .ok_or(DecryptError::DecryptFailure)
    }
}
