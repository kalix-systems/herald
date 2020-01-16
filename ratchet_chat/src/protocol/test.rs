use super::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Stores {
    ratchets: HashMap<sig::PublicKey, dr::DoubleRatchet>,
    sigs: HashMap<UserId, sig::SigChain>,
    convos: HashMap<ConversationId, UserId>,
    pending: HashMap<sig::PublicKey, HashMap<PayloadId, Payload>>,
    keys: HashMap<kx::PublicKey, HashMap<dr::Counter, aead::Key>>,
}

impl StoreLike for Stores {
    type Error = void::Void;
}

impl dr::KeyStore for Stores {
    fn get_key(
        &mut self,
        pk: kx::PublicKey,
        ix: dr::Counter,
    ) -> Result<Option<aead::Key>, Self::Error> {
        Ok(self.keys.get(&pk).and_then(|h| h.get(&ix).cloned()))
    }

    fn store_key(
        &mut self,
        pk: kx::PublicKey,
        ix: dr::Counter,
        key: aead::Key,
    ) -> Result<(), Self::Error> {
        self.keys.entry(pk).or_default().insert(ix, key);
        Ok(())
    }

    fn remove_key(
        &mut self,
        pk: kx::PublicKey,
        ix: dr::Counter,
    ) -> Result<(), Self::Error> {
        let should_del: bool;
        if let Some(h) = self.keys.get_mut(&pk) {
            let _ = h.remove(&ix);
            should_del = h.is_empty();
        } else {
            should_del = false;
        }

        if should_del {
            self.keys.remove(&pk);
        }

        Ok(())
    }

    fn contains_pk(
        &mut self,
        pk: kx::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(self.keys.contains_key(&pk))
    }
}

impl RatchetStore for Stores {
    fn get_ratchet(
        &mut self,
        with: sig::PublicKey,
    ) -> Result<Option<dr::DoubleRatchet>, Self::Error> {
        todo!()
    }

    fn store_ratchet(
        &mut self,
        with: sig::PublicKey,
        ratchet: dr::DoubleRatchet,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

impl SigStore for Stores {
    fn start_sigchain(
        &mut self,
        init: Signed<UserId>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn extend_sigchain(
        &mut self,
        from: UserId,
        update: Signed<sig::SigUpdate>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn get_sigchain(
        &mut self,
        of: UserId,
    ) -> Result<Option<sig::SigChain>, Self::Error> {
        todo!()
    }

    fn active_keys(
        &mut self,
        of: UserId,
    ) -> Result<Vec<sig::PublicKey>, Self::Error> {
        todo!()
    }

    fn key_is_valid(
        &mut self,
        key: sig::PublicKey,
        valid_for: UserId,
    ) -> Result<bool, Self::Error> {
        todo!()
    }

    fn all_active_keys(&mut self) -> Result<Vec<sig::PublicKey>, Self::Error> {
        todo!()
    }
}
impl ConversationStore for Stores {
    fn add_to_convo(
        &mut self,
        cid: ConversationId,
        members: Vec<UserId>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn left_convo(
        &mut self,
        cid: ConversationId,
        from: UserId,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn get_members(
        &mut self,
        cid: ConversationId,
    ) -> Result<Vec<UserId>, Self::Error> {
        todo!()
    }

    fn member_of(
        &mut self,
        cid: ConversationId,
        uid: UserId,
    ) -> Result<bool, Self::Error> {
        todo!()
    }
}

impl PendingStore for Stores {
    fn add_pending_payload(
        &mut self,
        id: PayloadId,
        payload: Payload,
        to: &Vec<sig::PublicKey>,
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

    fn get_pending_to(
        &mut self,
        to: sig::PublicKey,
    ) -> Result<Vec<(PayloadId, Payload)>, Self::Error> {
        todo!()
    }
}

#[test]
fn init_recv() {}
