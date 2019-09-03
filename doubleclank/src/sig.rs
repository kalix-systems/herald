use sodiumoxide::crypto::sign;

pub struct SecretKey(sign::SecretKey);
pub struct PublicKey(sign::PublicKey);
pub struct Signature(sign::Signature);

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

pub struct Signed<'a> {
    msg: &'a [u8],
    signer: &'a PublicKey,
    signature: Signature,
}

impl SecretKey {
    fn sign(&self, msg: &[u8]) -> Signature {
        Signature(sign::sign_detached(msg, &self.0))
    }
}

impl PublicKey {
    fn verify(&self, msg: &[u8], sig: &Signature) -> bool {
        sign::verify_detached(&sig.0, msg, &self.0)
    }
}

impl KeyPair {
    pub fn sign<'a>(&'a self, msg: &'a [u8]) -> Signed<'a> {
        let signature = self.sec_key.sign(msg);
        Signed {
            msg,
            signature,
            signer: self.pub_key(),
        }
    }
}
