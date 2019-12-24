use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TaggedPrekey {
    pub key: sig::PublicKey,
    pub prekey: Signed<Prekey>,
}

#[derive(Clone, Copy, Debug)]
pub struct PrekeyReplace {
    pub new: Signed<Prekey>,
    pub old: Option<Prekey>,
}

impl Conn {
    pub async fn new_prekeys<Keys: Stream<Item = PrekeyReplace> + Send>(
        &mut self,
        keys: Keys,
    ) -> Result<new_prekeys::Res, Error> {
        let (insert_stmt, update_stmt, slot_stmt, is_valid_stmt) = try_join!(
            self.prepare_typed(sql!("add_prekey"), types![BYTEA, BYTEA, BYTEA, INT8, INT2]),
            self.prepare_typed(
                sql!("replace_prekey"),
                types![BYTEA, BYTEA, BYTEA, INT8, BYTEA]
            ),
            self.prepare_typed(sql!("prekey_slot"), types![BYTEA]),
            self.prepare_typed(sql!("key_is_valid"), types![BYTEA])
        )?;

        // in both cases, check that the signing key is valid

        // if old is some, replace it

        // if old is none, pick the smallest free slot or fail if one cannot be found
        let res: Result<(), Res<new_prekeys::Res>> = keys
            .map(Ok::<_, Res<new_prekeys::Res>>)
            .try_for_each_concurrent(10, |PrekeyReplace { new, old }| {
                let conn = &self;
                let (new, meta) = new.split();

                let insert_stmt = &insert_stmt;
                let update_stmt = &update_stmt;
                let slot_stmt = &slot_stmt;
                let is_valid_stmt = &is_valid_stmt;

                let signed_by_bytes = meta.signed_by().as_ref().to_vec();
                let sig_bytes = meta.sig().as_ref().to_vec();
                let ts = *meta.timestamp().as_i64();

                async move {
                    if !conn
                        .query_one(is_valid_stmt, params![meta.signed_by().as_ref()])
                        .await
                        .map_err(Error::from)
                        .map_err(Err)?
                        .get::<_, bool>(0)
                    {
                        return Err(Ok(new_prekeys::Res::DeadKey(new)));
                    }

                    let Prekey(new) = new;

                    match old {
                        Some(Prekey(old)) => {
                            conn.execute(
                                update_stmt,
                                params![new.as_ref(), signed_by_bytes, sig_bytes, ts, old.as_ref()],
                            )
                            .await
                            .map_err(Error::from)
                            .map_err(Err)?;

                            Ok(())
                        }
                        None => {
                            let slots = conn
                                .query(slot_stmt, params![signed_by_bytes])
                                .await
                                .map_err(Error::from)
                                .map_err(Err)?
                                .into_iter()
                                .map(|row| row.get::<_, i16>(0) as u8);

                            let slot = if slots.len() == 256 {
                                return Err(Ok(new_prekeys::Res::NoSlotAvailable(Prekey(new))));
                            } else {
                                let mut prev = 0u8;

                                for (ix, s) in slots.enumerate() {
                                    if (ix as u8) != s {
                                        break;
                                    } else {
                                        prev += 1;
                                    }
                                }

                                prev as i16
                            };

                            let num_updated = conn
                                .execute(
                                    insert_stmt,
                                    params![new.as_ref(), signed_by_bytes, sig_bytes, ts, slot],
                                )
                                .await
                                .map_err(Error::from)
                                .map_err(Err)?;

                            if num_updated != 1 {
                                return Err(Ok(new_prekeys::Res::Redundant(Prekey(new))));
                            }

                            Ok(())
                        }
                    }
                }
            })
            .await;

        match res {
            Err(v @ Ok(_)) => return v,
            Err(Err(e)) => return Err(e),
            _ => {}
        };

        Ok(new_prekeys::Res::Success)
    }

    pub async fn get_random_prekeys<Keys: Stream<Item = sig::PublicKey> + Send>(
        &mut self,
        keys: Keys,
    ) -> Res<Vec<TaggedPrekey>> {
        let prekeys = Mutex::new(Vec::new());

        let stmt = self
            .prepare_typed(sql!("get_random_prekeys"), types![BYTEA])
            .await?;

        keys.map(Ok::<_, Error>)
            .try_for_each_concurrent(10, |k| {
                let conn = &self;
                let stmt = &stmt;
                let prekeys = &prekeys;

                async move {
                    let row = conn.query_one(stmt, params![k.as_ref()]).await?;

                    let prekey = Prekey::from_slice(row.get("key")).ok_or(Error::InvalidKey)?;

                    let sig = sig::Signature::from_slice(row.get("signature"))
                        .ok_or(Error::InvalidSig)?;

                    let signed_by = sig::PublicKey::from_slice(row.get("signed_by"))
                        .ok_or(Error::InvalidKey)?;

                    let timestamp = Time::from(row.get::<_, i64>("ts"));

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::get_client;
    use crate::{w, wa};
    use futures::stream::iter;
    use serial_test_derive::serial;
    use std::convert::TryInto;
    use womp::*;

    #[tokio::test]
    #[serial]
    async fn add_and_get_prekey() {
        let mut client = wa!(get_client());

        let uid: UserId = w!("a".try_into());
        let kp = sig::KeyPair::gen_new();
        let init = kp.sign(uid);

        let pre_kp = sig::KeyPair::gen_new();
        let pre = w!(Prekey::from_slice(pre_kp.public_key().as_ref()));

        let signed_pre = kp.sign(pre);

        let replace = PrekeyReplace {
            old: None,
            new: signed_pre,
        };

        match wa!(client.new_prekeys(iter(vec![replace]))) {
            new_prekeys::Res::DeadKey(_) => {}
            _ => panic!(),
        };

        wa!(client.new_user(init));

        match wa!(client.new_prekeys(iter(vec![replace]))) {
            new_prekeys::Res::Success => {}
            _ => panic!(),
        };

        let tagged = wa!(client.get_random_prekeys(iter(vec![*kp.public_key()])));
        assert_eq!(tagged.len(), 1);
        assert_eq!(
            tagged[0],
            TaggedPrekey {
                key: *kp.public_key(),
                prekey: signed_pre
            }
        );

        match wa!(client.new_prekeys(iter(vec![replace]))) {
            new_prekeys::Res::Redundant(_) => {}
            _ => panic!(),
        };

        let pre_kp2 = sig::KeyPair::gen_new();
        let pre2 = w!(Prekey::from_slice(pre_kp2.public_key().as_ref()));

        let signed_pre2 = kp.sign(pre2);

        let replace = PrekeyReplace {
            old: Some(pre),
            new: signed_pre2,
        };

        match wa!(client.new_prekeys(iter(vec![replace]))) {
            new_prekeys::Res::Success => {}
            _ => panic!(),
        };

        let tagged = wa!(client.get_random_prekeys(iter(vec![*kp.public_key()])));
        assert_eq!(tagged.len(), 1);
        assert_eq!(
            tagged[0],
            TaggedPrekey {
                key: *kp.public_key(),
                prekey: signed_pre2
            }
        );
    }
}
