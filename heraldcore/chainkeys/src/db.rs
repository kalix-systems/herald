use super::*;
use byteorder::*;
use kdf_ratchet::*;
use rusqlite::named_params;

pub struct Tx<'a>(rusqlite::Transaction<'a>);

pub fn with_tx<E, O, F>(f: F) -> Result<O, E>
where
    F: FnOnce(&mut Tx) -> Result<O, E>,
    E: From<rusqlite::Error>,
{
    let mut conn = CK_CONN.lock();
    with_tx_from_conn(&mut conn, f)
}

pub fn with_tx_from_conn<E, O, F>(
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
        gen: u32,
        state: &RatchetState,
    ) -> Result<(), rusqlite::Error> {
        let mut store_stmt = self.prepare_cached(include_str!("sql/store_ratchet_state.sql"))?;
        store_stmt.execute_named(named_params! {
            "@cid": cid,
            "@pk": pk.as_ref(),
            "@gen": gen,
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
        gen: u32,
        ix: u64,
        key: kcl::aead::Key,
    ) -> Result<(), rusqlite::Error> {
        let mut store_stmt = self.prepare_cached(include_str!("sql/store_derived_key.sql"))?;
        let ts = Time::now();
        store_stmt.execute_named(named_params! {
            "@cid": cid,
            "@pk": pk.as_ref(),
            "@gen": gen,
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
        gen: u32,
    ) -> Result<RatchetState, ChainKeysError> {
        let mut get_stmt = self.prepare_cached(include_str!("sql/get_ratchet_state.sql"))?;
        let (raw_ix, raw_base_key, raw_ratchet_key) = get_stmt
            .query_map_named(
                named_params! {"@cid": cid, "@pk": pk.as_ref(), "@gen": gen},
                |row| {
                    Ok((
                        row.get::<_, Vec<u8>>("next_ix")?,
                        row.get::<_, Vec<u8>>("base_key")?,
                        row.get::<_, Vec<u8>>("ratchet_key")?,
                    ))
                },
            )?
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

    pub fn get_recent_ratchet(
        &self,
        cid: ConversationId,
        pk: sig::PublicKey,
    ) -> Result<Option<(u32, RatchetState)>, ChainKeysError> {
        let mut get_stmt = self.prepare_cached(include_str!("sql/get_recent_ratchet.sql"))?;
        let qres = get_stmt
            .query_map_named(named_params! {"@cid": cid, "@pk": pk.as_ref()}, |row| {
                Ok((
                    row.get("generation")?,
                    row.get::<_, Vec<u8>>("next_ix")?,
                    row.get::<_, Vec<u8>>("base_key")?,
                    row.get::<_, Vec<u8>>("ratchet_key")?,
                ))
            })?
            .next()
            .transpose()?;
        Ok(
            if let Some((gen, raw_ix, raw_base_key, raw_ratchet_key)) = qres {
                let ix = if raw_ix.len() != 8 {
                    return Err(ChainKeysError::StoreCorrupted);
                } else {
                    LE::read_u64(&raw_ix)
                };

                let base_key = kcl::hash::Key::from_slice(&raw_base_key)
                    .ok_or(ChainKeysError::StoreCorrupted)?;

                let ratchet_key = RatchetKey::from_slice(&raw_ratchet_key)
                    .ok_or(ChainKeysError::StoreCorrupted)?;

                let ratchet = RatchetState::mk(ix, base_key, ratchet_key);
                Some((gen, ratchet))
            } else {
                None
            },
        )
    }

    pub fn get_generation(
        &self,
        cid: ConversationId,
        pk: sig::PublicKey,
    ) -> Result<u32, ChainKeysError> {
        let mut get_stmt = self.prepare_cached(include_str!("sql/get_generation.sql"))?;
        let gen = get_stmt
            .query_map_named(named_params! {"@cid": cid, "@pk": pk.as_ref()}, |row| {
                Ok(row.get("generation")?)
            })?
            .next()
            .ok_or(ChainKeysError::NoneError(loc!()))??;
        Ok(gen)
    }

    pub fn get_derived_key(
        &self,
        cid: ConversationId,
        pk: sig::PublicKey,
        gen: u32,
        ix: u64,
    ) -> Result<Option<kcl::aead::Key>, ChainKeysError> {
        let mut stmt = self.prepare_cached(include_str!("sql/get_derived_key.sql"))?;
        let res = stmt
            .query_map_named(
                named_params! {
                    "@cid": cid,
                    "@pk": pk.as_ref(),
                    "@gen": gen,
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
        gen: u32,
        cipher: kdf_ratchet::Cipher,
    ) -> Result<Option<Decrypted>, ChainKeysError> {
        use kdf_ratchet::DecryptionResult::*;

        let res = if let Some(k) = self.get_derived_key(cid, pk, gen, cipher.index)? {
            cipher.open_with(k)
        } else {
            let mut state = self.get_ratchet_state(cid, pk, gen)?;
            self.store_ratchet_state(cid, pk, gen, &state)?;
            state.open(cipher)
        };

        match res {
            Success { extra_keys, ad, pt } => {
                for (ix, key) in extra_keys {
                    self.store_derived_key(cid, pk, gen, ix, key)?;
                }

                Ok(Some(Decrypted { ad, pt }))
            }
            Failed { extra_keys } => {
                for (ix, key) in extra_keys {
                    self.store_derived_key(cid, pk, gen, ix, key)?;
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
    ) -> Result<(u32, kdf_ratchet::Cipher, Option<RatchetState>), ChainKeysError> {
        if let Some((gen, mut ratchet)) = self.get_recent_ratchet(cid, pk)? {
            let (ix, key, cipher) = ratchet.seal(ad, msg).destruct();

            self.store_ratchet_state(cid, pk, gen, &ratchet)?;
            self.store_derived_key(cid, pk, gen, ix, key)?;

            Ok((gen, cipher, None))
        } else {
            let ret_ratchet = RatchetState::gen_new();
            let gen = self.get_generation(cid, pk)? + 1;
            let mut ratchet = ret_ratchet.clone();

            let (ix, key, cipher) = ratchet.seal(ad, msg).destruct();
            self.store_ratchet_state(cid, pk, gen, &ratchet)?;
            self.store_derived_key(cid, pk, gen, ix, key)?;

            Ok((gen, cipher, Some(ret_ratchet)))
        }
    }

    pub fn deprecate_before(
        &mut self,
        cid: ConversationId,
        pk: sig::PublicKey,
        gen: u32,
    ) -> Result<(), rusqlite::Error> {
        let mut store_stmt = self.prepare_cached(include_str!("sql/deprecate_before.sql"))?;
        store_stmt.execute_named(named_params! {
            "@cid": cid,
            "@pk": pk.as_ref(),
            "@gen": gen
        })?;
        Ok(())
    }

    pub fn deprecate_all_in_convo(
        &mut self,
        cid: ConversationId,
        pk: sig::PublicKey,
    ) -> Result<(), rusqlite::Error> {
        let mut store_stmt = self.prepare_cached(include_str!("sql/deprecate_all_in_convo.sql"))?;
        store_stmt.execute_named(named_params! {
            "@cid": cid,
            "@pk": pk.as_ref()
        })?;
        Ok(())
    }

    pub fn deprecate_all(
        &mut self,
        pk: sig::PublicKey,
    ) -> Result<(), rusqlite::Error> {
        let mut store_stmt = self.prepare_cached(include_str!("sql/deprecate_all.sql"))?;
        store_stmt.execute_named(named_params! {
            "@pk": pk.as_ref(),
        })?;
        Ok(())
    }
}
