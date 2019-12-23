use super::*;

pub struct TaggedPrekey {
    pub key: sig::PublicKey,
    pub prekey: Signed<Prekey>,
}

pub struct PrekeyReplace {
    pub new: Signed<Prekey>,
    pub old: Option<Prekey>,
}

impl Conn {
    pub async fn new_prekeys<Keys: Stream<Item = PrekeyReplace> + Send>(
        &mut self,
        keys: Keys,
    ) -> Result<new_prekeys::Res, Error> {
        let (insert_stmt, update_stmt) = try_join!(
            self.prepare_typed("TODO", types![BYTEA, BYTEA, BYTEA, INT8]),
            self.prepare_typed("TODO", types![BYTEA, BYTEA, BYTEA, BYTEA, BYTEA, INT8])
        )?;

        keys.map(Ok::<_, Error>)
            .try_for_each_concurrent(10, |PrekeyReplace { new, old }| {
                let conn = &self;
                let insert_stmt = &insert_stmt;
                let update_stmt = &update_stmt;

                async move {
                    match old {
                        Some(Prekey(old)) => todo!(),
                        None => todo!(),
                    }
                }
            })
            .await?;

        unimplemented!()
    }

    pub async fn get_random_prekeys<Keys: Stream<Item = sig::PublicKey> + Send>(
        &mut self,
        keys: Keys,
    ) -> Res<Vec<TaggedPrekey>> {
        let prekeys = Mutex::new(Vec::new());

        let stmt = self.prepare_typed("TODO", types![BYTEA]).await?;

        keys.map(Ok::<_, Error>)
            .try_for_each_concurrent(10, |k| {
                let conn = &self;
                let stmt = &stmt;
                let prekeys = &prekeys;

                async move {
                    let row = conn.query_one(stmt, params![k.as_ref()]).await?;

                    let prekey = Prekey::from_slice(row.get("prekey")).ok_or(Error::InvalidKey)?;

                    let sig = sig::Signature::from_slice(row.get("signature"))
                        .ok_or(Error::InvalidSig)?;
                    let signed_by = sig::PublicKey::from_slice(row.get("signed_by"))
                        .ok_or(Error::InvalidKey)?;
                    let timestamp = Time::from(row.get::<_, i64>("timestamp"));

                    let meta = SigMeta::new(sig, signed_by, timestamp);

                    prekeys.lock().push(TaggedPrekey {
                        key: k,
                        prekey: Signed::from((prekey, meta)),
                    });

                    Ok(())
                }
            })
            .await?;

        Ok(prekeys.into_inner())
    }
}
