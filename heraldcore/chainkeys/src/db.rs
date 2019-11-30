use super::*;
use byteorder::*;
use kdf_ratchet::*;
use rusqlite::named_params;

pub struct Tx<'a>(rusqlite::Transaction<'a>);

pub fn with_tx<F, E, O>(f: F) -> Result<O, E>
where
    F: FnOnce(&mut Tx) -> Result<O, E>,
    E: From<rusqlite::Error>,
{
    let mut conn = CK_CONN.lock();
    with_tx_from_conn(&mut conn, f)
    // let mut tx = Tx(conn.transaction()?);
    // let o = f(&mut tx)?;
    // tx.0.commit()?;
    // Ok(o)
}

pub fn with_tx_from_conn<F, E, O>(
    conn: &mut rusqlite::Connection,
    f: F,
) -> Result<O, E>
where
    F: FnOnce(&mut Tx) -> Result<O, E>,
    E: From<rusqlite::Error>,
{
    let mut tx = Tx(conn.transaction()?);
    let o = f(&mut tx)?;
    tx.0.commit()?;
    Ok(o)
}

impl<'a> std::ops::Deref for Tx<'a> {
    type Target = rusqlite::Transaction<'a>;

    fn deref(&self) -> &rusqlite::Transaction<'a> {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for Tx<'a> {
    fn deref_mut(&mut self) -> &mut rusqlite::Transaction<'a> {
        &mut self.0
    }
}

impl Tx<'_> {
    pub fn store_ratchet_state(
        &mut self,
        cid: ConversationId,
        pk: sig::PublicKey,
        state: &RatchetState,
    ) -> Result<(), rusqlite::Error> {
        let mut store_stmt = self.prepare_cached(include_str!("sql/store_ratchet_state.sql"))?;
        store_stmt.execute_named(named_params! {
            "@cid": cid,
            "@pk": pk.as_ref(),
            "@ix": &state.ix().to_le_bytes() as &[u8],
            "@base_key": state.base_key().as_ref(),
            "@ratchet_key": state.ratchet_key().as_ref()
        })?;
        Ok(())
    }

    pub fn store_derived_key(
        &mut self,
        cid: ConversationId,
        pk: sig::PublicKey,
        ix: u64,
        key: kcl::aead::Key,
    ) -> Result<(), rusqlite::Error> {
        let mut store_stmt = self.prepare_cached(include_str!("sql/store_derived_key.sql"))?;
        let ts = Time::now();
        store_stmt.execute_named(named_params! {
            "@cid": cid,
            "@pk": pk.as_ref(),
            "@ix": &ix.to_le_bytes() as &[u8],
            "@msg_key": key.as_ref(),
            "@ts": ts.as_i64()
        })?;
        Ok(())
    }

    pub fn get_ratchet_state(
        &self,
        cid: ConversationId,
        pk: sig::PublicKey,
    ) -> Result<RatchetState, ChainKeysError> {
        let mut get_stmt = self.prepare_cached(include_str!("sql/get_ratchet_state.sql"))?;
        let (raw_ix, raw_base_key, raw_ratchet_key) = get_stmt
            .query_map_named(named_params! {"@cid": cid, "@pk": pk.as_ref()}, |row| {
                Ok((
                    row.get::<_, Vec<u8>>("next_ix")?,
                    row.get::<_, Vec<u8>>("base_key")?,
                    row.get::<_, Vec<u8>>("ratchet_key")?,
                ))
            })?
            .next()
            .ok_or(ChainKeysError::NoneError(loc!()))??;

        let ix = if raw_ix.len() != 8 {
            return Err(ChainKeysError::StoreCorrupted);
        } else {
            LE::read_u64(&raw_ix)
        };

        let base_key =
            kcl::hash::Key::from_slice(&raw_base_key).ok_or(ChainKeysError::StoreCorrupted)?;

        let ratchet_key =
            RatchetKey::from_slice(&raw_ratchet_key).ok_or(ChainKeysError::StoreCorrupted)?;

        Ok(RatchetState::mk(ix, base_key, ratchet_key))
    }

    pub fn get_derived_key(
        &self,
        cid: ConversationId,
        pk: sig::PublicKey,
        ix: u64,
    ) -> Result<Option<kcl::aead::Key>, ChainKeysError> {
        let mut stmt = self.prepare_cached(include_str!("sql/get_derived_key.sql"))?;
        let res = stmt
            .query_map_named(
                named_params! {
                    "@cid": cid,
                    "@pk": pk.as_ref(),
                    "@ix": &ix.to_le_bytes() as &[u8]
                },
                |row| Ok(row.get::<_, Vec<u8>>("msg_key")?),
            )?
            .next()
            .transpose()?
            .map(|raw_key| {
                kcl::aead::Key::from_slice(&raw_key).ok_or(ChainKeysError::StoreCorrupted)
            })
            .transpose()?;
        Ok(res)
    }

    pub fn open_msg(
        &mut self,
        cid: ConversationId,
        pk: sig::PublicKey,
        cipher: kdf_ratchet::Cipher,
    ) -> Result<Option<Decrypted>, ChainKeysError> {
        use kdf_ratchet::DecryptionResult::*;

        let res = if let Some(k) = self.get_derived_key(cid, pk, cipher.index)? {
            cipher.open_with(k)
        } else {
            let mut state = self.get_ratchet_state(cid, pk)?;
            self.store_ratchet_state(cid, pk, &state)?;
            state.open(cipher)
        };

        match res {
            Success { extra_keys, ad, pt } => {
                for (ix, key) in extra_keys {
                    self.store_derived_key(cid, pk, ix, key)?;
                }

                Ok(Some(Decrypted { ad, pt }))
            }
            Failed { extra_keys } => {
                for (ix, key) in extra_keys {
                    self.store_derived_key(cid, pk, ix, key)?;
                }

                Ok(None)
            }
            // TODO: include these fields in error msg
            IndexTooHigh { .. } => Err(ChainKeysError::StoreCorrupted),
        }
    }

    pub fn seal_msg(
        &mut self,
        cid: ConversationId,
        pk: sig::PublicKey,
        ad: Bytes,
        msg: BytesMut,
    ) -> Result<kdf_ratchet::Cipher, ChainKeysError> {
        let mut ratchet = self.get_ratchet_state(cid, pk)?;
        let (ix, key, cipher) = ratchet.seal(ad, msg).destruct();
        self.store_derived_key(cid, pk, ix, key)?;
        self.store_ratchet_state(cid, pk, &ratchet)?;
        Ok(cipher)
    }
}
