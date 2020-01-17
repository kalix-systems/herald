use crate::*;
use coremacros::w;
use herald_common::{kson, sig};
use ratchet_chat::protocol::{Payload, PayloadId, PendingStore};
use rusqlite::NO_PARAMS;

impl<'conn> PendingStore for Conn<'conn> {
    fn add_pending_payload(
        &mut self,
        id: PayloadId,
        payload: Payload,
        to: &[sig::PublicKey],
    ) -> Result<(), Self::Error> {
        w!(self.add_payload(&id, &payload));
        w!(self.add_pending(&id, to));
        Ok(())
    }

    fn get_pending_payload(
        &mut self,
        id: PayloadId,
    ) -> Result<Option<Payload>, Self::Error> {
        let mut stmt = st!(self, "pending", "payload");
        let params = np!("@id": id.as_slice());

        let mut res = w!(stmt.query_map_named(params, |row| { row.get::<_, Vec<u8>>("payload") }));

        let raw: Vec<u8> = w!(ok_none!(res.next()));

        Ok(w!(kson::from_slice(&raw)))
    }

    fn del_pending(
        &mut self,
        id: PayloadId,
        to: sig::PublicKey,
    ) -> Result<(), Self::Error> {
        let mut stmt = st!(self, "pending", "del");
        let params = np!("@id": id.as_slice(), "@recipient": to.as_ref());

        w!(stmt.execute_named(params));
        w!(self.gc_pending());

        Ok(())
    }
}

impl<'conn> Conn<'conn> {
    fn add_pending(
        &self,
        id: &PayloadId,
        recips: &[sig::PublicKey],
    ) -> Result<(), rusqlite::Error> {
        let mut stmt = st!(self, "pending", "add_pending");

        for recip in recips {
            let params = np!("@id": id.as_slice(), "@recipient": recip.as_ref());
            w!(stmt.execute_named(params));
        }

        Ok(())
    }

    fn add_payload(
        &self,
        id: &PayloadId,
        payload: &Payload,
    ) -> Result<(), rusqlite::Error> {
        let mut stmt = st!(self, "pending", "add_payload");

        let params = np!("@id": id.as_slice(), "@payload": kson::to_vec(payload));
        w!(stmt.execute_named(params));
        Ok(())
    }

    fn gc_pending(&self) -> Result<(), rusqlite::Error> {
        let mut stmt = st!(self, "pending", "gc");
        w!(stmt.execute(NO_PARAMS));
        Ok(())
    }
}
