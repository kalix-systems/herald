use sodiumoxide::crypto::sign;

#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct SecretKey(sign::SecretKey);
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct PublicKey(sign::PublicKey);
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Signature(sign::Signature);

#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct KeyPair {
    sec_key: SecretKey,
    pub_key: PublicKey,
}

impl KeyPair {
    pub fn new() -> Self {
        let (pub_key, sec_key) = sign::gen_keypair();
        KeyPair {
            pub_key: PublicKey(pub_key),
            sec_key: SecretKey(sec_key),
        }
    }

    pub fn pub_key(&self) -> &PublicKey {
        &self.pub_key
    }

    pub fn sec_key(&self) -> &SecretKey {
        &self.sec_key
    }
}

impl SecretKey {
    pub fn sign(&self, msg: &[u8]) -> Signature {
        Signature(sign::sign_detached(msg, &self.0))
    }
}

impl PublicKey {
    pub fn verify(&self, msg: &[u8], sig: &Signature) -> bool {
        sign::verify_detached(&sig.0, msg, &self.0)
    }
}
