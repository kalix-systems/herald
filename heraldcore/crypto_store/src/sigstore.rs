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

impl<'conn> Conn<'conn> {
    fn key_deprecated(
        &self,
        key: &PK,
        of: &UserId,
    ) -> Result<bool, rusqlite::Error> {
        let mut stmt = st!(self, "sigchain", "key_deprecated");

        let params = np!("@key": key.as_ref(), "user_id": of);
        stmt.query_row_named(params, |row| row.get(0))
    }

    fn key_endorsed(
        &self,
        key: &PK,
        of: &UserId,
    ) -> Result<bool, rusqlite::Error> {
        let mut stmt = st!(self, "sigchain", "key_endorsed");

        let params = np!("@key": key.as_ref(), "user_id": of);
        stmt.query_row_named(params, |row| row.get(0))
    }

    fn key_is_genesis(
        &self,
        key: &PK,
        of: &UserId,
    ) -> Result<bool, rusqlite::Error> {
        let mut stmt = st!(self, "sigchain", "key_is_genesis");

        let params = np!("@key": key.as_ref(), "user_id": of);
        stmt.query_row_named(params, |row| row.get(0))
    }
}

impl<'conn> Conn<'conn> {
    fn sigchain_genesis(
        &self,
        of: UserId,
    ) -> Result<Option<Signed<UserId>>, crate::Error> {
        let mut stmt = st!(self, "sigchain", "genesis");
        let params = np!("@user_id": of);

        let mut res = w!(stmt.query_map_named(params, |row| {
            let raw_sig = w!(row.get::<_, Vec<u8>>("signature"));
            let sig = ok_none!(Sig::from_slice(&raw_sig));

            let raw_signed_by = w!(row.get::<_, Vec<u8>>("signed_by"));
            let signed_by = ok_none!(PK::from_slice(&raw_signed_by));

            let ts = w!(row.get("ts"));

            let meta = SigMeta::new(sig, signed_by, ts);

            Ok(Some((of, meta).into()))
        }));

        Ok(w!(ok_none!(res.next())))
    }

    fn deprecations(
        &self,
        of: UserId,
    ) -> Result<Vec<Signed<sig::SigUpdate>>, crate::Error> {
        let mut stmt = st!(self, "sigchain", "deprecations");
        let params = np!("@user_id": of);

        struct Raw {
            raw_sig: Vec<u8>,
            raw_signed_by: Vec<u8>,
            raw_key: Vec<u8>,
            ts: Time,
        };

        let res = w!(stmt.query_map_named(params, |row| {
            Ok(Raw {
                raw_sig: w!(row.get("signature")),
                raw_signed_by: w!(row.get("signed_by")),
                raw_key: w!(row.get("key")),
                ts: w!(row.get("ts")),
            })
        }));

        res.map(|r| {
            let Raw {
                raw_sig,
                raw_signed_by,
                raw_key,
                ts,
            } = w!(r);

            let meta = {
                let sig = w!(Sig::from_slice(&raw_sig).ok_or(BadSignature));
                let signed_by = w!(PK::from_slice(&raw_signed_by).ok_or(BadKey));

                SigMeta::new(sig, signed_by, ts)
            };

            let key = w!(PK::from_slice(&raw_key).ok_or(BadKey));

            Ok((sig::SigUpdate::Deprecate(key), meta).into())
        })
        .collect()
    }

    fn endorsements(
        &self,
        of: UserId,
    ) -> Result<Vec<Signed<sig::SigUpdate>>, crate::Error> {
        let mut stmt = st!(self, "sigchain", "endorsements");
        let params = np!("@user_id": of);

        struct Raw {
            inner_raw_sig: Vec<u8>,
            inner_raw_signed_by: Vec<u8>,
            inner_ts: Time,
            outer_raw_sig: Vec<u8>,
            outer_raw_signed_by: Vec<u8>,
            outer_ts: Time,
        }

        let res = w!(stmt.query_map_named(params, |row| {
            Ok(Raw {
                inner_raw_sig: w!(row.get("inner_signature")),
                inner_raw_signed_by: w!(row.get("inner_signed_by")),
                inner_ts: w!(row.get("inner_ts")),
                outer_raw_sig: w!(row.get("outer_signature")),
                outer_raw_signed_by: w!(row.get("outer_signed_by")),
                outer_ts: w!(row.get("outer_ts")),
            })
        }));

        res.map(|r| {
            let Raw {
                inner_raw_sig,
                inner_raw_signed_by,
                inner_ts,
                outer_raw_signed_by,
                outer_raw_sig,
                outer_ts,
            } = w!(r);

            let endorsement = {
                let inner_sig = w!(Sig::from_slice(&inner_raw_sig).ok_or(BadSignature));
                let inner_signed_by = w!(PK::from_slice(&inner_raw_signed_by).ok_or(BadKey));

                let inner_meta = SigMeta::new(inner_sig, inner_signed_by, inner_ts);

                sig::SigUpdate::Endorse((of, inner_meta).into())
            };

            let outer_sig = w!(Sig::from_slice(&outer_raw_sig).ok_or(BadSignature));
            let outer_signed_by = w!(PK::from_slice(&outer_raw_signed_by).ok_or(BadKey));

            let outer_meta = SigMeta::new(outer_sig, outer_signed_by, outer_ts);

            Ok((endorsement, outer_meta).into())
        })
        .collect()
    }
}
