use super::*;

impl<'conn> Conn<'conn> {
    pub(super) fn sigchain_genesis(
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

    pub(super) fn deprecations(
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

    pub(super) fn endorsements(
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
