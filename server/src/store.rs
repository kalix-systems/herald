use chrono::prelude::*;
use failure::*;
use herald_common::*;
use queue_file::QueueFile;
use semalock::Semalock;
use serde_cbor::{from_reader as deserialize_from, to_writer as serialize_into};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Store {
    rootdir: PathBuf,
}
fn queue_error(_: queue_file::Error) -> Error {
    format_err!("queuefile error")
}

impl Store {
    fn extend_path<P: AsRef<Path>>(&self, ext: &P) -> PathBuf {
        let mut out = self.rootdir.clone();
        out.push(ext.as_ref());
        out
    }

    pub fn sigkeys_for(&self, uid: UserIdRef) -> PathBuf {
        let mut path = self.extend_path(&"sigkeys");
        path.push(uid);
        path
    }

    pub fn pending_for(&self, key: sig::PublicKey) -> PathBuf {
        let mut path = self.extend_path(&"pending");
        path.push(format!("{:x?}", key));
        path
    }

    pub fn prekeys_for(&self, key: sig::PublicKey) -> PathBuf {
        let mut path = self.extend_path(&"prekeys");
        path.push(format!("{:x?}", key));
        path
    }

    pub fn with_lock<P, F, R>(&self, ext: &P, with: F) -> Result<R, Error>
    where
        P: AsRef<Path>,
        F: FnOnce(&mut File) -> R,
    {
        let path = self.extend_path(ext);

        let mut lock = Semalock::new(&path).map_err(|e| {
            format_err!(
                "failed to create lock on metadata for path {:?}, error was {}",
                path,
                e
            )
        })?;

        let r = lock.with(move |l| with(&mut l.file)).map_err(|e| {
            format_err!(
                "failed to acquire lock on metadata for path {:?}, error was {}",
                path,
                e
            )
        })?;

        Ok(r)
    }

    pub fn create_user(&self, uid: UserIdRef) -> Result<(), Error> {
        fs::create_dir_all(self.sigkeys_for(uid))?;
        Ok(())
    }

    pub fn add_key(
        &self,
        uid: UserIdRef,
        key: sig::PublicKey,
        meta: sig::PKMeta,
    ) -> Result<bool, Error> {
        if !meta.key_is_valid(key) {
            return Ok(false);
        }
        let mut path = self.sigkeys_for(uid);
        path.push(format!("{:x?}", key.as_ref()));
        self.with_lock(&path, |f| serialize_into(f, &meta))??;

        File::create(self.pending_for(key))?;
        File::create(self.prekeys_for(key))?;

        Ok(true)
    }

    pub fn read_key(&self, uid: UserIdRef, key: sig::PublicKey) -> Result<sig::PKMeta, Error> {
        let mut path = self.sigkeys_for(uid);
        path.push(format!("{:x?}", key.as_ref()));
        Ok(self.with_lock(&path, |f| deserialize_from(f))??)
    }

    pub fn deprecate_key(
        &self,
        uid: UserIdRef,
        key: Signed<sig::PublicKey>,
    ) -> Result<bool, Error> {
        let (k, sig) = key.split();
        let signer = *sig.signed_by();
        if signer == k {
            bail!("can't deprecate key with itself");
        } else if !sig.verify_sig(k.as_ref()) {
            bail!("can't deprecate with invalid signature");
        } else if self.read_key(uid, signer)?.key_is_valid(signer) {
            bail!("can't deprecate key with invalid key");
        }
        let mut meta = self.read_key(uid, k)?;
        meta.deprecate(sig);

        let mut path = self.sigkeys_for(uid);
        path.push(format!("{:x?}", k.as_ref()));
        self.with_lock(&path, |f| serialize_into(f, &meta))??;
        Ok(true)
    }

    pub fn key_is_valid(&self, uid: UserIdRef, key: sig::PublicKey) -> Result<bool, Error> {
        self.read_key(uid, key).map(|pkm| pkm.key_is_valid(key))
    }

    pub fn add_prekey(&self, key: sig::PublicKey, pre: sealed::PublicKey) -> Result<bool, Error> {
        if !pre.verify_sig() || *pre.signed_by() != key {
            return Ok(false);
        }
        let mut qf = QueueFile::open(self.prekeys_for(key)).map_err(queue_error)?;
        qf.add(&serde_cbor::to_vec(&pre)?).map_err(queue_error)?;
        Ok(true)
    }

    pub fn get_prekey(&self, key: sig::PublicKey) -> Result<sealed::PublicKey, Error> {
        let mut qf = QueueFile::open(self.prekeys_for(key)).map_err(queue_error)?;
        let raw = qf
            .peek()
            .map_err(queue_error)?
            .ok_or_else(|| format_err!("no prekeys stored"))?;
        qf.remove().map_err(queue_error)?;
        Ok(serde_cbor::from_slice(&raw)?)
    }

    // pub fn read_meta(&self, uid: UserIdRef) -> Result<User, Error> {
    //     let u: User = self.with_lock(&uid, |f| deserialize_from::<User, &mut File>(f))??;
    //     Ok(u)
    // }

    // /// Adds a signed key to the user's metadata, which can now be used to sign more keys.
    // /// Returns `Err(_)` if the user or device doesn't exist, or if the filesystem failed to write.
    // pub fn add_key(
    //     &self,
    //     uid: UserId,
    //     did: DeviceId,
    //     new_key: RawKey,
    //     date: DateTime<Utc>,
    //     signature: RawSig,
    // ) -> Result<Signed<RawKey>, Error> {
    //     let mut meta = self.read_meta(uid)?;
    //     let signed = meta.new_signed(VERIFIER, did, new_key, date, signature)?;
    //     let created = CreatedKey::new(signed.clone());
    //     meta.add_new_key(created);
    //     let mut path = self.rootdir.clone();
    //     path.push(uid.to_string());
    //     serialize_into(File::open(path)?, &meta)?;
    //     Ok(signed)
    // }

    // /// Adds a signed key to the user's metadata, which can now be used to sign more keys.
    // /// Returns `Err(_)` if the user or device doesn't exist, or if the filesystem failed to write.
    // pub fn deprecate_key(
    //     &self,
    //     uid: UserId,
    //     did: DeviceId,
    //     new_key: RawKey,
    //     date: DateTime<Utc>,
    //     signature: RawSig,
    // ) -> Result<Signed<RawKey>, Error> {
    //     let mut meta = self.read_meta(uid)?;
    //     let signed = meta.new_signed(VERIFIER, did, new_key, date, signature)?;
    //     let created = CreatedKey::new(signed.clone());
    //     meta.add_new_key(created);
    //     let mut path = self.rootdir.clone();
    //     path.push(uid.as_str());
    //     serialize_into(File::open(path)?, &meta)?;
    //     Ok(signed)
    // }
}
