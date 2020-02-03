use crate::*;
use coremacros::w;
use herald_common::{kson, sig};
use ratchet_chat::{protocol::RatchetStore, ratchet::double as dr};

impl<'conn> RatchetStore for Conn<'conn> {
    fn get_ratchet(
        &mut self,
        with: sig::PublicKey,
    ) -> Result<Option<dr::DoubleRatchet>, Self::Error> {
        let bytes = ok_none!(w!(self.get_ratchet_raw(with)));
        let ratchet = w!(kson::from_slice(&bytes));

        Ok(Some(ratchet))
    }

    fn store_ratchet(
        &mut self,
        with: sig::PublicKey,
        ratchet: dr::DoubleRatchet,
    ) -> Result<(), Self::Error> {
        let ratchet_bytes = kson::to_vec(&ratchet);

        w!(self.store_ratchet_raw(with, ratchet_bytes));

        Ok(())
    }
}

impl<'conn> Conn<'conn> {
    fn get_ratchet_raw(
        &mut self,
        with: sig::PublicKey,
    ) -> Result<Option<Vec<u8>>, rusqlite::Error> {
        let mut stmt = st!(self, "ratchet", "get");

        let params = np!("@public_key": with.as_ref());
        let mut res = w!(stmt.query_map_named(params, |row| row.get::<_, Vec<u8>>("ratchet")));

        Ok(w!(res.next().transpose()))
    }

    fn store_ratchet_raw(
        &mut self,
        with: sig::PublicKey,
        ratchet_bytes: Vec<u8>,
    ) -> Result<(), rusqlite::Error> {
        let mut stmt = st!(self, "ratchet", "store");

        let params = np!("@public_key": with.as_ref(), "@ratchet": ratchet_bytes);

        w!(stmt.execute_named(params));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::in_memory;
    use coremacros::womp;

    #[test]
    fn ratchet_ops() {
        let mut conn = in_memory();
        let mut conn = Conn::from(conn.transaction().expect(womp!()));

        let pk = *sig::KeyPair::gen_new().public();

        let dummy = |i| vec![i; 32];

        assert!(conn.get_ratchet_raw(pk).expect(womp!()).is_none());

        conn.store_ratchet_raw(pk, dummy(1)).expect(womp!());

        assert_eq!(conn.get_ratchet_raw(pk).expect(womp!()), Some(dummy(1)));

        conn.store_ratchet_raw(pk, dummy(2)).expect(womp!());

        assert_eq!(conn.get_ratchet_raw(pk).expect(womp!()), Some(dummy(2)));
    }
}
