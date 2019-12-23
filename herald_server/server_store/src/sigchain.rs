use super::*;

impl Conn {
    pub async fn get_sigchain(
        &mut self,
        user: UserId,
    ) -> Res<Option<sig::SigChain>> {
        let (rows, dep_rows) = {
            let conn = &self;

            try_join!(
                async move {
                    let stmt = conn
                        .prepare_typed(sql!("key_creations"), types![TEXT])
                        .await?;
                    conn.query(&stmt, params![user.as_str()]).await
                },
                async move {
                    let stmt = conn
                        .prepare_typed(sql!("key_deprecations"), types![TEXT])
                        .await?;
                    conn.query(&stmt, params![user.as_str()]).await
                }
            )
        }?;

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
            let sig = row.get("signature");
            let signed_by = row.get("signed_by");
            let timestamp: i64 = row.get("timestamp");

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

        let mut sig_chain = Vec::with_capacity(rows.len() + dep_rows.len());

        for row in rows {
            let inner_meta: SigMeta = get_inner_meta(&row)?;
            let inner = sig::SigUpdate::Endorse((user, inner_meta).into());

            let meta: SigMeta = get_meta(&row)?;
            let update = Signed::from((inner, meta));

            sig_chain.push(update);
        }

        for row in dep_rows {
            let pk = sig::PublicKey::from_slice(row.get("key")).ok_or(Error::InvalidKey)?;
            let meta = get_meta(&row)?;
            let update = Signed::from((sig::SigUpdate::Deprecate(pk), meta));

            sig_chain.push(update);
        }

        sig_chain.sort_unstable_by(|a, b| a.timestamp().cmp(b.timestamp()));

        Ok(Some(sig::SigChain { initial, sig_chain }))
    }

    pub async fn add_to_sigchain(
        &mut self,
        new: Signed<sig::SigUpdate>,
    ) -> Result<PKIResponse, Error> {
        use sig::SigUpdate::*;

        let (update, meta) = new.split();

        let tx = self.transaction().await?;

        let (key_created, key_deprecated) = {
            let tx = &tx;
            try_join!(
                async move {
                    Ok::<bool, Error>(
                        tx.query_one(sql!("key_created"), params![meta.signed_by().as_ref()])
                            .await?
                            .get::<_, bool>(0),
                    )
                },
                async move {
                    Ok::<bool, Error>(
                        tx.query_one(sql!("key_deprecated"), params![meta.signed_by().as_ref()])
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
                            meta.timestamp().as_i64(),
                            meta.sig().as_ref(),
                        ],
                    )
                    .await?;

                if num_updated != 1 {
                    return Ok(PKIResponse::Redundant);
                }
            }

            Deprecate(pk) => {
                let signer_key = meta.signed_by();

                let dep_stmt = tx
                    .prepare_typed(sql!("deprecate_key"), types![INT8, BYTEA, BYTEA, BYTEA])
                    .await?;

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
