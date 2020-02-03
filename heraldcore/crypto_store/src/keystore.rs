use crate::*;
use coremacros::w;
use kcl::{aead, kx};
use ratchet_chat::ratchet::double::*;

impl<'conn> KeyStore for Conn<'conn> {
    fn get_key(
        &mut self,
        pk: kx::PublicKey,
        ix: Counter,
    ) -> Result<Option<aead::Key>, Self::Error> {
        let mut stmt = st!(self, "keystore", "get");
        let params = np!("@public_key": pk.as_ref(), "@ix": ix);

        let mut res =
            w!(stmt.query_map_named(params, |row| { Ok(w!(row.get::<_, Vec<u8>>("key"))) }));

        let raw_key = w!(ok_none!(res.next()));

        Ok(Some(w!(
            aead::Key::from_slice(&raw_key).ok_or(Error::BadKey)
        )))
    }

    fn store_key(
        &mut self,
        pk: kx::PublicKey,
        ix: Counter,
        key: aead::Key,
    ) -> Result<(), Self::Error> {
        let mut stmt = st!(self, "keystore", "store");
        let params = np!("@key": key.as_ref(), "@public_key": pk.as_ref(), "@ix": ix);

        w!(stmt.execute_named(params));

        Ok(())
    }

    fn remove_key(
        &mut self,
        pk: kx::PublicKey,
        ix: Counter,
    ) -> Result<(), Self::Error> {
        let mut stmt = st!(self, "keystore", "remove");
        let params = np!("@public_key": pk.as_ref(), "@ix": ix);
        w!(stmt.execute_named(params));

        Ok(())
    }

    fn contains_pk(
        &mut self,
        pk: kx::PublicKey,
    ) -> Result<bool, Self::Error> {
        let mut stmt = st!(self, "keystore", "exists");
        let params = np!("@public_key": pk.as_ref());

        let mut res = w!(stmt.query_map_named(params, |row| Ok(w!(row.get::<_, bool>(0)))));

        Ok(w!(res.next().transpose()).unwrap_or(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::in_memory;
    use coremacros::womp;

    #[test]
    fn key_ops() {
        let mut conn = in_memory();
        let mut conn = Conn::from(conn.transaction().expect(womp!()));

        let pk = *kx::KeyPair::gen_new().public();

        let key = aead::Key::new();

        let ix1 = 0;
        let ix2 = 1;

        assert!(!conn.contains_pk(pk).expect(womp!()));
        assert!(conn.get_key(pk, ix1).expect(womp!()).is_none());

        conn.store_key(pk, ix1, key.clone()).expect(womp!());

        assert!(conn.contains_pk(pk).expect(womp!()));
        assert_eq!(conn.get_key(pk, ix1).expect(womp!()), Some(key.clone()));

        assert!(conn.get_key(pk, ix2).expect(womp!()).is_none());

        conn.remove_key(pk, ix1).expect(womp!());

        assert!(!conn.contains_pk(pk).expect(womp!()));
        assert!(conn.get_key(pk, ix1).expect(womp!()).is_none());
    }
}
