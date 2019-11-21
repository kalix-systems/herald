use super::*;
use byteorder::*;

pub struct Tx<'a>(rusqlite::Transaction<'a>);

pub fn with_tx<F, E, O>(f: F) -> Result<O, E>
where
    F: FnOnce(&mut Tx) -> Result<O, E>,
    E: From<rusqlite::Error>,
{
    let mut conn = CK_CONN.lock();
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
        let res = self
            .prepare_cached(include_str!("sql/get_derived_key.sql"))?
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
}
