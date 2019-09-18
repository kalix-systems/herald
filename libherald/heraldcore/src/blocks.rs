use chainmail::block::*;
use std::collections::BTreeSet;

pub(crate) struct Blocks;

impl BlockStore for Blocks {
    // stores a key, does not mark key as used
    fn store_key(&mut self, hash: BlockHash, key: ChainKey) {}

    // we'll want to implement some kind of gc strategy to collect keys marked used
    // if they're less than an hour old we should keep them, otherwise delete
    // could run on a schedule, or every time we call get_unused, or something else
    fn mark_used<'a, I: Iterator<Item = &'a BlockHash>>(&self, blocks: I) {}

    // this should *not* mark keys as used
    fn get_keys<'a, I: Iterator<Item = &'a BlockHash>>(
        &self,
        blocks: I,
    ) -> Option<BTreeSet<ChainKey>> {
        None
    }

    // this should *not* mark keys as used
    fn get_unused(&self) -> Vec<(BlockHash, ChainKey)> {
        vec![]
    }
}
