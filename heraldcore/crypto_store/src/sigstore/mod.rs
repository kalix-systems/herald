use crate::*;
use coremacros::w;
use herald_common::{
    sig::{self, PublicKey as PK, Signature as Sig},
    SigMeta, Signed, Time, UserId,
};
use ratchet_chat::protocol::SigStore;
use rusqlite::NO_PARAMS;
use std::ops::Not;

use Error::{BadKey, BadSignature};

mod chain_getters;
mod checks;

#[cfg(test)]
mod tests;

impl<'conn> SigStore for Conn<'conn> {
    fn start_sigchain(
        &mut self,
        init: Signed<UserId>,
    ) -> Result<(), Self::Error> {
        let mut stmt = st!(self, "sigchain", "start");

        let sig = init.sig();
        let params = np!(
            "@user_id": init.data(),
            "@ts": init.timestamp(),
            "@signature": sig.as_ref(),
            "@signed_by": init.signed_by().as_ref()
        );

        w!(stmt.execute_named(params));

        Ok(())
    }

    fn extend_sigchain(
        &mut self,
        from: UserId,
        update: Signed<sig::SigUpdate>,
    ) -> Result<(), Self::Error> {
        use sig::SigUpdate::*;
        let (update, meta) = update.split();
        let outer_ts = meta.timestamp();
        let outer_sig = meta.sig();
        let outer_signed_by = meta.signed_by();

        match update {
            Endorse(signed_uid) => {
                let (uid, inner_meta) = signed_uid.split();

                debug_assert_eq!(uid, from);

                let mut stmt = st!(self, "sigchain", "endorse");

                let inner_ts = inner_meta.timestamp();
                let inner_sig = inner_meta.sig();
                let inner_signed_by = inner_meta.signed_by();

                let params = np!(
                    "@outer_ts": outer_ts,
                    "@outer_signature": outer_sig.as_ref(),
                    "@outer_signed_by": outer_signed_by.as_ref(),
                    "@inner_ts": inner_ts,
                    "@inner_signature": inner_sig.as_ref(),
                    "@inner_signed_by": inner_signed_by.as_ref(),
                    "@user_id": uid
                );

                w!(stmt.execute_named(params));
            }
            Deprecate(key) => {
                let mut stmt = st!(self, "sigchain", "deprecate");

                let params = np!(
                    "@ts": outer_ts,
                    "@signature": outer_sig.as_ref(),
                    "@signed_by": outer_signed_by.as_ref(),
                    "@key": key.as_ref(),
                    "@user_id": from
                );

                w!(stmt.execute_named(params));
            }
        };

        Ok(())
    }

    fn get_sigchain(
        &mut self,
        of: UserId,
    ) -> Result<Option<sig::SigChain>, Self::Error> {
        let initial = ok_none!(w!(self.sigchain_genesis(of)));

        let sig_chain = {
            let mut updates = w!(self.endorsements(of));
            let mut deprecations = w!(self.deprecations(of));
            updates.append(&mut deprecations);

            updates.sort_unstable_by(|a, b| a.timestamp().cmp(b.timestamp()));

            updates
        };

        Ok(Some(sig::SigChain { initial, sig_chain }))
    }

    fn key_is_valid(
        &mut self,
        key: PK,
        valid_for: UserId,
    ) -> Result<bool, Self::Error> {
        Ok(
            (w!(self.key_endorsed(&key, &valid_for)) || w!(self.key_is_genesis(&key, &valid_for)))
                && w!(self.key_deprecated(&key, &valid_for)).not(),
        )
    }

    fn all_active_keys(&mut self) -> Result<Vec<PK>, Self::Error> {
        let mut stmt = st!(self, "sigchain", "all_active_keys");

        let res = w!(stmt.query_map(NO_PARAMS, |row| { row.get::<_, Vec<u8>>("key") }));
        res.map(|raw_key| {
            let raw_key = w!(raw_key);
            let key = w!(PK::from_slice(&raw_key).ok_or(BadKey));

            Ok(key)
        })
        .collect()
    }
}
