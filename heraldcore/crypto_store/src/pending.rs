use crate::*;
use coremacros::w;
use herald_common::sig;
use ratchet_chat::protocol::{Payload, PayloadId, PendingStore};

impl<'conn> PendingStore for Conn<'conn> {
    fn add_pending_payload(
        &mut self,
        id: PayloadId,
        payload: Payload,
        to: &[sig::PublicKey],
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn get_pending_payload(
        &mut self,
        id: PayloadId,
    ) -> Result<Option<Payload>, Self::Error> {
        todo!()
    }

    fn del_pending(
        &mut self,
        id: PayloadId,
        to: sig::PublicKey,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
