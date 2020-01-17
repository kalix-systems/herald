use crate::*;
use coremacros::w;
use herald_common::{kson, sig};
use ratchet_chat::{protocol::RatchetStore, ratchet::double as dr};

impl<'conn> RatchetStore for Conn<'conn> {
    fn get_ratchet(
        &mut self,
        with: sig::PublicKey,
    ) -> Result<Option<dr::DoubleRatchet>, Self::Error> {
        let mut stmt = st!(self, "ratchet", "get");

        let params = np!("@public_key": with.as_ref());
        let mut res = w!(stmt.query_map_named(params, |row| row.get::<_, Vec<u8>>("ratchet")));

        let bytes = ok_none!(w!(res.next().transpose()));

        let ratchet = w!(kson::from_slice(&bytes));

        Ok(Some(ratchet))
    }

    fn store_ratchet(
        &mut self,
        with: sig::PublicKey,
        ratchet: dr::DoubleRatchet,
    ) -> Result<(), Self::Error> {
        let mut stmt = st!(self, "ratchet", "store");

        let params = np!("@public_key": with.as_ref(), "@ratchet": kson::to_vec(&ratchet));

        w!(stmt.execute_named(params));

        Ok(())
    }
}