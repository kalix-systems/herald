use super::*;
use byteorder::*;
use kdf_ratchet::*;

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
        state: &RatchetState,
    ) -> Result<(), rusqlite::Error> {
        let mut store_stmt = self.prepare_cached(include_str!("sql/store_ratchet_state.sql"))?;
        store_stmt.execute(params![
            cid,
            &state.ix().to_le_bytes() as &[u8],
            state.base_key().as_ref(),
            state.ratchet_key().as_ref()
        ])?;
        Ok(())
    }

    pub fn store_derived_key(
        &mut self,
        cid: ConversationId,
        ix: u64,
        key: kcl::aead::Key,
    ) -> Result<(), rusqlite::Error> {
        let mut store_stmt = self.prepare_cached(include_str!("sql/store_derived_key.sql"))?;
        let ts = Time::now();
        store_stmt.execute(params![cid, &ix.to_le_bytes() as &[u8], key.as_ref(), ts.0])?;
        Ok(())
    }

    pub fn mark_used(
        &mut self,
        cid: ConversationId,
        ix: u64,
    ) -> Result<(), ChainKeysError> {
        let mut set_stmt = self.prepare_cached(include_str!("sql/mark_used.sql"))?;
        set_stmt.execute(params![cid, &ix.to_le_bytes() as &[u8]])?;
        Ok(())
    }

    pub fn get_ratchet_state(
        &self,
        cid: ConversationId,
    ) -> Result<RatchetState, ChainKeysError> {
        let mut get_stmt = self.prepare_cached(include_str!("sql/get_ratchet_state.sql"))?;
        let (raw_ix, raw_base_key, raw_ratchet_key) = get_stmt
            .query_map(params![cid], |row| {
                Ok((
                    row.get::<_, Vec<u8>>(0)?,
                    row.get::<_, Vec<u8>>(1)?,
                    row.get::<_, Vec<u8>>(2)?,
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
        ix: u64,
    ) -> Result<Option<kcl::aead::Key>, ChainKeysError> {
        let mut stmt = self.prepare_cached(include_str!("sql/get_derived_key.sql"))?;
        let res = stmt
            .query_map(params![cid, &ix.to_le_bytes() as &[u8]], |row| {
                Ok(row.get::<_, Vec<u8>>(0)?)
            })?
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
        cipher: kdf_ratchet::Cipher,
    ) -> Result<Option<Decrypted>, ChainKeysError> {
        use kdf_ratchet::DecryptionResult::*;

        let res = if let Some(k) = self.get_derived_key(cid, cipher.index)? {
            let ix = cipher.index;
            let r0 = cipher.open_with(k);
            if let Success { .. } = &r0 {
                self.mark_used(cid, ix)?;
            }
            r0
        } else {
            let mut state = self.get_ratchet_state(cid)?;
            let res = state.open(cipher);
            self.store_ratchet_state(cid, &state)?;
            res
        };

        match res {
            Success { extra_keys, ad, pt } => {
                if let Some((ix, _)) = extra_keys.last() {
                    self.mark_used(cid, *ix)?;
                }

                for (ix, key) in extra_keys {
                    self.store_derived_key(cid, ix, key)?;
                }

                Ok(Some(Decrypted { ad, pt }))
            }
            Failed { extra_keys } => {
                for (ix, key) in extra_keys {
                    self.store_derived_key(cid, ix, key)?;
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
        ad: Bytes,
        msg: BytesMut,
    ) -> Result<kdf_ratchet::Cipher, ChainKeysError> {
        let mut ratchet = self.get_ratchet_state(cid)?;
        let (ix, key, cipher) = ratchet.seal(ad, msg).destruct();
        self.store_derived_key(cid, ix, key)?;
        self.mark_used(cid, ix)?;
        self.store_ratchet_state(cid, &ratchet)?;
        Ok(cipher)
    }
}
