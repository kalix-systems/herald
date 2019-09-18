use crate::{
    abort_err,
    db::{DBTable, Database},
    errors::HErr,
};
use chainmail::block::*;
use rusqlite::{params, NO_PARAMS};
use std::collections::BTreeSet;

#[derive(Default)]
pub(crate) struct Keys {
    db: Database,
}

impl DBTable for Keys {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/keys/create_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn drop_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/keys/drop_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists() -> Result<bool, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/keys/table_exists.sql"))?;
        Ok(stmt.exists(NO_PARAMS)?)
    }

    fn reset() -> Result<(), HErr> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        tx.execute(include_str!("sql/keys/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/keys/create_table.sql"), NO_PARAMS)?;
        tx.commit()?;
        Ok(())
    }
}

impl BlockStore for Keys {
    // stores a key, does not mark key as used
    fn store_key(&mut self, hash: BlockHash, key: ChainKey) {
        self.db
            .execute(
                include_str!("sql/keys/store_key.sql"),
                params![hash.as_ref(), key.as_ref()],
            )
            .expect("failed to store key");
    }

    // we'll want to implement some kind of gc strategy to collect keys marked used
    // if they're less than an hour old we should keep them, otherwise delete
    // could run on a schedule, or every time we call get_unused, or something else
    fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(&self, blocks: I) {
        let mut stmt = abort_err!(self.db.prepare(include_str!("sql/keys/mark_used.sql")));

        for block in blocks {
            abort_err!(stmt.execute(params![block.as_ref()]));
        }
    }

    // this should *not* mark keys as used
    fn get_keys<'a, I: Iterator<Item = &'a BlockHash>>(
        &self,
        blocks: I,
    ) -> Option<BTreeSet<ChainKey>> {
        let mut stmt = self
            .db
            .prepare(include_str!("sql/keys/get_keys.sql"))
            .ok()?;

        let mut keys: BTreeSet<ChainKey> = BTreeSet::new();

        for block in blocks.map(|block| block.as_ref()) {
            let key: Vec<u8> = stmt.query_row(params![block], |row| row.get(0)).ok()?;
            keys.insert(ChainKey::from_slice(key.as_slice())?);
        }

        Some(keys)
    }

    // this should *not* mark keys as used
    fn get_unused(&self) -> Vec<(BlockHash, ChainKey)> {
        let mut stmt = abort_err!(self.db.prepare(include_str!("sql/keys/get_unused.sql")));

        let results = stmt
            .query_map(NO_PARAMS, |row| {
                Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, Vec<u8>>(1)?))
            })
            .unwrap();

        let mut pairs = vec![];

        for res in results {
            let (raw_hash, raw_key) = res.unwrap();
            pairs.push((
                BlockHash::from_slice(raw_hash.as_slice()).unwrap(),
                ChainKey::from_slice(raw_key.as_slice()).unwrap(),
            ));
        }

        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test_derive::serial;
    use womp::*;

    #[test]
    #[serial]
    fn create_drop_exists() {
        Database::reset_all().expect(womp!());
        // drop twice, it shouldn't panic on multiple drops
        Keys::drop_table().expect(womp!());
        Keys::drop_table().expect(womp!());

        Keys::create_table().expect(womp!());
        assert!(Keys::exists().expect(womp!()));
        Keys::create_table().expect(womp!());
        assert!(Keys::exists().expect(womp!()));
        Keys::drop_table().expect(womp!());
        assert!(!Keys::exists().expect(womp!()));
        Keys::reset().expect(womp!());
    }

    #[test]
    #[serial]
    fn blockstore() {
        Database::reset_all().expect(womp!());
        let mut handle = Keys::default();

        let blockhash1 = BlockHash::from_slice(vec![1; BLOCKHASH_BYTES].as_slice()).expect(womp!());
        let blockhash2 = BlockHash::from_slice(vec![2; BLOCKHASH_BYTES].as_slice()).expect(womp!());
        let chainkey1 = ChainKey::from_slice(vec![1; CHAINKEY_BYTES].as_slice()).expect(womp!());
        let chainkey2 = ChainKey::from_slice(vec![2; CHAINKEY_BYTES].as_slice()).expect(womp!());

        handle.store_key(blockhash1, (&chainkey1).clone());
        handle.store_key(blockhash2, (&chainkey2).clone());

        let known_keys: BTreeSet<ChainKey> = vec![(&chainkey1).clone(), (&chainkey2).clone()]
            .into_iter()
            .collect();

        let keys = handle
            .get_keys(vec![blockhash1, blockhash2].iter())
            .expect(womp!());

        assert_eq!(keys.len(), 2);
        assert_eq!(known_keys, keys);

        handle.mark_used(vec![blockhash1].iter());

        let unused: Vec<_> = handle.get_unused().into_iter().collect();
        assert_eq!(unused.len(), 1);
        assert_eq!(unused, vec![(blockhash2, chainkey2)]);
    }
}
