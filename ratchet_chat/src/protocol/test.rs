use super::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct Stores {
    ratchets: HashMap<sig::PublicKey, dr::DoubleRatchet>,
    sigs: HashMap<UserId, sig::SigChain>,
    convos: HashMap<ConversationId, HashSet<UserId>>,
    pending_by_id: HashMap<PayloadId, (Payload, HashSet<sig::PublicKey>)>,
    pending_by_to: HashMap<sig::PublicKey, HashSet<PayloadId>>,
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
        Ok(self.ratchets.get(&with).cloned())
    }

    fn store_ratchet(
        &mut self,
        with: sig::PublicKey,
        ratchet: dr::DoubleRatchet,
    ) -> Result<(), Self::Error> {
        self.ratchets.insert(with, ratchet);
        Ok(())
    }
}

impl SigStore for Stores {
    fn start_sigchain(
        &mut self,
        initial: Signed<UserId>,
    ) -> Result<(), Self::Error> {
        let sigchain = sig::SigChain {
            initial,
            sig_chain: vec![],
        };
        self.sigs.insert(*initial.data(), sigchain);
        Ok(())
    }

    fn extend_sigchain(
        &mut self,
        from: UserId,
        update: Signed<sig::SigUpdate>,
    ) -> Result<(), Self::Error> {
        if let Some(s) = self.sigs.get_mut(&from) {
            s.sig_chain.push(update);
        }
        Ok(())
    }

    fn get_sigchain(
        &mut self,
        of: UserId,
    ) -> Result<Option<sig::SigChain>, Self::Error> {
        Ok(self.sigs.get(&of).cloned())
    }

    fn key_is_valid(
        &mut self,
        key: sig::PublicKey,
        valid_for: UserId,
    ) -> Result<bool, Self::Error> {
        if let Some(chain) = self.sigs.get(&valid_for) {
            Ok(chain.active_keys().contains(&key))
        } else {
            Ok(false)
        }
    }

    fn all_active_keys(&mut self) -> Result<Vec<sig::PublicKey>, Self::Error> {
        let keys = self
            .sigs
            .values()
            .flat_map(|chain| chain.active_keys().into_iter())
            .collect();
        Ok(keys)
    }
}
impl ConversationStore for Stores {
    fn add_to_convo(
        &mut self,
        cid: ConversationId,
        members: Vec<UserId>,
    ) -> Result<(), Self::Error> {
        let c = self.convos.entry(cid).or_default();
        for member in members {
            c.insert(member);
        }
        Ok(())
    }

    fn left_convo(
        &mut self,
        cid: ConversationId,
        from: UserId,
    ) -> Result<(), Self::Error> {
        if let Some(c) = self.convos.get_mut(&cid) {
            c.remove(&from);
        }
        Ok(())
    }

    fn get_members(
        &mut self,
        cid: ConversationId,
    ) -> Result<Vec<UserId>, Self::Error> {
        Ok(self
            .convos
            .get(&cid)
            .map(|c| c.iter().copied().collect())
            .unwrap_or(vec![]))
    }

    fn member_of(
        &mut self,
        cid: ConversationId,
        uid: UserId,
    ) -> Result<bool, Self::Error> {
        Ok(self
            .convos
            .get(&cid)
            .map(|c| c.contains(&uid))
            .unwrap_or(false))
    }
}

impl PendingStore for Stores {
    fn add_pending_payload(
        &mut self,
        id: PayloadId,
        payload: Payload,
        to: &Vec<sig::PublicKey>,
    ) -> Result<(), Self::Error> {
        let (_, by_id) = self
            .pending_by_id
            .entry(id)
            .or_insert((payload, HashSet::new()));
        for key in to {
            self.pending_by_to.entry(*key).or_default().insert(id);
            by_id.insert(*key);
        }
        Ok(())
    }

    fn get_pending_payload(
        &mut self,
        id: PayloadId,
    ) -> Result<Option<Payload>, Self::Error> {
        Ok(self.pending_by_id.get(&id).map(|(p, _)| p.clone()))
    }

    fn del_pending(
        &mut self,
        id: PayloadId,
        to: sig::PublicKey,
    ) -> Result<(), Self::Error> {
        if let Some(pends) = self.pending_by_to.get_mut(&to) {
            pends.remove(&id);
            if pends.is_empty() {
                self.pending_by_to.remove(&to);
            }
        }
        if let Some((_, by_id)) = self.pending_by_id.get_mut(&id) {
            by_id.remove(&to);
            if by_id.is_empty() {
                self.pending_by_id.remove(&id);
            }
        }
        Ok(())
    }
}

#[test]
fn init_recv() {}
