use crate::user::*;
use bincode::{deserialize_from, serialize_into};
use chrono::prelude::*;
use failure::*;
use ring::signature::*;
use std::fs::File;
use std::path::PathBuf;

pub static VERIFIER: &EdDSAParameters = &ED25519;

pub struct Store {
    pub rootdir: PathBuf,
    // TODO: add arcmap for transactions
}

impl Store {
    pub fn read_meta(&self, uid: UserId) -> Result<UserMeta, Error> {
        let mut path = self.rootdir.clone();
        path.push(uid.to_string());
        let u = deserialize_from(File::open(path)?)?;
        Ok(u)
    }

    /// Adds a signed key to the user's metadata, which can now be used to sign more keys.
    /// Returns `Err(_)` if the user or device doesn't exist, or if the filesystem failed to write.
    pub fn add_key(
        &self,
        uid: UserId,
        did: DeviceId,
        new_key: RawKey,
        date: DateTime<Utc>,
        signature: RawSig,
    ) -> Result<Signed<RawKey>, Error> {
        let mut meta = self.read_meta(uid)?;
        let signed = meta.new_signed(VERIFIER, did, new_key, date, signature)?;
        let created = CreatedKey::new(signed.clone());
        meta.add_new_key(created);
        let mut path = self.rootdir.clone();
        path.push(uid.to_string());
        serialize_into(File::open(path)?, &meta)?;
        Ok(signed)
    }

    /// Adds a signed key to the user's metadata, which can now be used to sign more keys.
    /// Returns `Err(_)` if the user or device doesn't exist, or if the filesystem failed to write.
    pub fn deprecate_key(
        &self,
        uid: UserId,
        did: DeviceId,
        new_key: RawKey,
        date: DateTime<Utc>,
        signature: RawSig,
    ) -> Result<Signed<RawKey>, Error> {
        let mut meta = self.read_meta(uid)?;
        let signed = meta.new_signed(VERIFIER, did, new_key, date, signature)?;
        let created = CreatedKey::new(signed.clone());
        meta.add_new_key(created);
        let mut path = self.rootdir.clone();
        path.push(uid.as_str());
        serialize_into(File::open(path)?, &meta)?;
        Ok(signed)
    }
}
