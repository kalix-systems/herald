use bincode::deserialize_from;
use chrono::prelude::*;
use failure::*;
use ring::signature::VerificationAlgorithm;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};

pub type UserId = u64;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum DeviceId {
    Original,
    Verified(u64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Signed<T> {
    pub data: T,
    pub signer: DeviceId,
    pub sig: Vec<u8>,
}

type RawKey = Vec<u8>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusUpdate {
    pub did: DeviceId,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Key {
    pub raw: RawKey,
    pub creation: Option<Signed<StatusUpdate>>,
    pub deprecated: Option<Signed<StatusUpdate>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMeta {
    pub original: RawKey,
    pub orig_depr: Option<Signed<StatusUpdate>>,
    pub verified_keys: Vec<Key>,
}

impl UserMeta {
    pub fn get_key(&self, did: DeviceId) -> Option<Key> {
        match did {
            DeviceId::Original => Some(Key {
                raw: self.original.clone(),
                creation: None,
                deprecated: self.orig_depr.clone(),
            }),
            DeviceId::Verified(i) => self.verified_keys.get(i as usize).cloned(),
        }
    }

    // /// Checks a signature `sig` on message `msg` using the public key from `did`.
    // /// Returns `Ok(true)` if the key exists, is valid, and the
    // pub fn check_sig<Msg: AsRef<[u8]>, Sig: AsRef<[u8]>>(
    //     &self,
    //     did: DeviceId,
    //     msg: Msg,
    //     sig: Sig,
    // ) -> Result<bool, Error> {
    //     let key = self.get_key(did).ok_or(format_err!(
    //         "couldn't find key associated with device {:?}",
    //         did
    //     ))?;
    //     unimplemented!()
    // }
}

pub struct Store<V> {
    pub rootdir: PathBuf,
    pub verifier: V,
}

impl<V: VerificationAlgorithm> Store<V> {
    pub fn read_meta(&self, uid: UserId) -> Result<UserMeta, Error> {
        let mut path = self.rootdir.clone();
        path.push(uid.to_string());
        let u = deserialize_from(File::open(path)?)?;
        Ok(u)
    }

    //     /// Adds a signed key to the user's metadata, which can now be used to sign more keys.
    //     /// Returns `Ok(true)` if the signature was correct and the new key was successfully added,
    //     /// `Ok(false)` if the signing process failed, and `Err(_)` if there were any other errors
    //     /// (e.g. user doesn't exist, device doesn't exist, fs failed to write)
    //     pub fn add_key(
    //         &self,
    //         uid: UserId,
    //         did: DeviceId,
    //         new_key: Input,
    //         signature: Input,
    //     ) -> Result<bool, Error> {
    //         let mut meta = self.read_meta(uid)?;
    //         let signing_key = meta.keys.get(&did).ok_or(format_err!("bad device id"))?;
    //         // match v.verify(signing_key, new_key, signature) {
    //         //     Ok(_) => {
    //         //         let new_id = meta.keys.len();
    //         //         meta.keys.insert(new_id, new_key);
    //         //     }
    //         //     Err(_) => Ok(false),
    //         // }
    //         unimplemented!()
    //     }
}
