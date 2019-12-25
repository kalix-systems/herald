use super::*;

impl Conn {
    pub async fn get_sigchain(
        &mut self,
        user: UserId,
    ) -> Res<Option<sig::SigChain>> {
        let updates_stmt = self.prepare_typed(sql!("sigchain"), types![TEXT]).await?;
        let rows = self.query(&updates_stmt, params![user.as_str()]).await?;

        let mut rows = rows.into_iter();

        let get_inner_meta = |row: &tokio_postgres::Row| -> Res<SigMeta> {
            let signed_by = row.get("key");
            let sig = row.get("inner_signature");
            let timestamp: i64 = row.get("inner_ts");

            Ok(SigMeta::new(
                sig::Signature::from_slice(sig).ok_or(Error::InvalidSig)?,
                sig::PublicKey::from_slice(signed_by).ok_or(Error::InvalidKey)?,
                Time::from(timestamp),
            ))
        };

        let get_meta = |row: &tokio_postgres::Row| -> Res<SigMeta> {
            let timestamp: i64 = row.get("outer_ts");
            let signed_by = row.get("outer_signed_by");
            let sig = row.get("outer_signature");

            Ok(SigMeta::new(
                sig::Signature::from_slice(sig).ok_or(Error::InvalidSig)?,
                sig::PublicKey::from_slice(signed_by).ok_or(Error::InvalidKey)?,
                Time::from(timestamp),
            ))
        };

        let initial = match rows.next() {
            None => return Ok(None),
            Some(initial) => {
                let meta = get_inner_meta(&initial)?;
                (user, meta).into()
            }
        };

        let mut sig_chain = Vec::with_capacity(rows.len());

        for row in rows {
            let meta = get_meta(&row)?;

            let update = if row.get::<_, bool>("is_creation") {
                let inner_meta = get_inner_meta(&row)?;
                let inner = sig::SigUpdate::Endorse((user, inner_meta).into());

                Signed::from((inner, meta))
            } else {
                let pk = sig::PublicKey::from_slice(row.get("key")).ok_or(Error::InvalidKey)?;

                Signed::from((sig::SigUpdate::Deprecate(pk), meta))
            };
            sig_chain.push(update);
        }

        Ok(Some(sig::SigChain { initial, sig_chain }))
    }

    pub async fn add_to_sigchain(
        &mut self,
        new: Signed<sig::SigUpdate>,
    ) -> Result<PKIResponse, Error> {
        use sig::SigUpdate::*;

        let (update, meta) = new.split();

        let tx = self.transaction().await?;

        let ((key_created, key_created_stmt), key_deprecated) = {
            let tx = &tx;
            try_join!(
                async move {
                    let stmt = tx.prepare_typed(sql!("key_created"), types![BYTEA]).await?;
                    Ok::<(bool, _), Error>((
                        tx.query_one(&stmt, params![meta.signed_by().as_ref()])
                            .await?
                            .get::<_, bool>(0),
                        stmt,
                    ))
                },
                async move {
                    let stmt = tx
                        .prepare_typed(sql!("key_deprecated"), types![BYTEA])
                        .await?;

                    Ok::<bool, Error>(
                        tx.query_one(&stmt, params![meta.signed_by().as_ref()])
                            .await?
                            .get::<_, bool>(0),
                    )
                },
            )?
        };

        if !key_created || key_deprecated {
            return Ok(PKIResponse::DeadKey);
        }

        match update {
            Endorse(signed_uid) => {
                let (uid, inner_meta) = signed_uid.split();

                let (user_key_stmt, endorsement_stmt) = try_join!(
                    tx.prepare_typed(sql!("add_user_key"), types!(TEXT, BYTEA)),
                    tx.prepare_typed(
                        sql!("add_endorsement"),
                        types!(BYTEA, BYTEA, INT8, BYTEA, BYTEA, INT8),
                    )
                )?;

                let num_updated = tx
                    .execute(
                        &user_key_stmt,
                        params![uid.as_str(), inner_meta.signed_by().as_ref()],
                    )
                    .await?;

                if num_updated != 1 {
                    return Ok(PKIResponse::Redundant);
                }

                let num_updated = tx
                    .execute(
                        &endorsement_stmt,
                        params![
                            inner_meta.signed_by().as_ref(),
                            inner_meta.sig().as_ref(),
                            inner_meta.timestamp().as_i64(),
                            meta.signed_by().as_ref(),
                            meta.sig().as_ref(),
                            meta.timestamp().as_i64(),
                        ],
                    )
                    .await?;

                if num_updated != 1 {
                    return Ok(PKIResponse::Redundant);
                }
            }

            Deprecate(pk) => {
                let signer_key = meta.signed_by();

                let (dep_stmt, inner_key_exists) = {
                    let tx = &tx;
                    try_join!(
                        tx.prepare_typed(sql!("deprecate_key"), types![INT8, BYTEA, BYTEA, BYTEA]),
                        async move {
                            Ok(tx
                                .query_one(&key_created_stmt, params![pk.as_ref()])
                                .await?
                                .get::<_, bool>(0))
                        }
                    )?
                };

                if !inner_key_exists {
                    return Ok(PKIResponse::DeadKey);
                }

                let num_updated = tx
                    .execute(
                        &dep_stmt,
                        params![
                            meta.timestamp().as_i64(),
                            signer_key.as_ref(),
                            meta.sig().as_ref(),
                            pk.as_ref(),
                        ],
                    )
                    .await?;

                if num_updated != 1 {
                    return Ok(PKIResponse::Redundant);
                }
            }
        }

        tx.commit().await?;

        Ok(PKIResponse::Success)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tests::get_client, w, wa};
    use serial_test_derive::serial;
    use std::convert::TryInto;
    use womp::*;

    #[tokio::test]
    #[serial]
    async fn add_and_leave() {
        use sig::SigUpdate::*;

        let mut client = wa!(get_client());

        let uid: UserId = w!("a".try_into());
        let kp = sig::KeyPair::gen_new();
        let init = kp.sign(uid);

        // should be empty initially
        assert!(wa!(client.get_sigchain(uid)).is_none());

        wa!(client.new_user(init));

        // should only have initial key
        match wa!(client.get_sigchain(uid)) {
            Some(sig::SigChain { initial, sig_chain }) => {
                assert_eq!(initial, init);
                assert!(sig_chain.is_empty());
            }
            _ => panic!(),
        };

        let second_kp = sig::KeyPair::gen_new();
        let first_update = Endorse(second_kp.sign(uid));
        let signed_first = kp.sign(first_update);

        let second_update = Deprecate(*second_kp.public_key());
        let signed_second = kp.sign(second_update);

        let third_kp = sig::KeyPair::gen_new();
        let third_update = Endorse(third_kp.sign(uid));
        let signed_third = second_kp.sign(third_update);

        // can't deprecate non-existent key
        match wa!(client.add_to_sigchain(signed_second)) {
            PKIResponse::DeadKey => {}
            _ => panic!(),
        };

        // should succeed
        match wa!(client.add_to_sigchain(signed_first)) {
            PKIResponse::Success => {}
            _ => panic!(),
        };

        // should have initial key and one endorsement
        match wa!(client.get_sigchain(uid)) {
            Some(sig::SigChain { initial, sig_chain }) => {
                assert_eq!(initial, init);
                assert_eq!(sig_chain.len(), 1);
                assert_eq!(sig_chain[0], signed_first);
            }
            _ => panic!(),
        };

        // should be redundant, we just did this
        match wa!(client.add_to_sigchain(signed_first)) {
            PKIResponse::Redundant => {}
            _ => panic!(),
        };

        // should have initial key and one update
        match wa!(client.get_sigchain(uid)) {
            Some(sig::SigChain { initial, sig_chain }) => {
                assert_eq!(initial, init);
                assert_eq!(sig_chain.len(), 1);
                assert_eq!(sig_chain[0], signed_first);
            }
            _ => panic!(),
        };

        // deprecation should now succeed...
        match wa!(client.add_to_sigchain(signed_second)) {
            PKIResponse::Success => {}
            _ => panic!(),
        };

        // ...but doing it twice is redundant
        match wa!(client.add_to_sigchain(signed_second)) {
            PKIResponse::Redundant => {}
            _ => panic!(),
        };

        // should have initial key, one endorsement, and one deprecation
        match wa!(client.get_sigchain(uid)) {
            Some(sig::SigChain { initial, sig_chain }) => {
                assert_eq!(initial, init);
                assert_eq!(sig_chain.len(), 2);
                assert_eq!(sig_chain[0], signed_first);
                assert_eq!(sig_chain[1], signed_second);
            }
            _ => panic!(),
        };

        // this should fail, as we're singing with a deprecated key
        match wa!(client.add_to_sigchain(signed_third)) {
            PKIResponse::DeadKey => {}
            _ => panic!(),
        };
    }
}
