use super::*;
use crate::slice_iter;

mod add_helpers;
use add_helpers::*;

#[derive(Debug, PartialEq)]
pub enum PushedTo {
    PushedTo {
        devs: Vec<sig::PublicKey>,
        push_id: i64,
    },
    Missing(SingleRecip),
    NoRecipients,
}

impl Conn {
    // should be done transactionally, returns Missing(r) for the first missing recip r
    // only adds to pending when it finds all devices
    pub async fn add_to_pending_and_get_valid_devs(
        &mut self,
        pairs: &[(&Recip, &Push)],
    ) -> Res<tokio::sync::mpsc::Receiver<(PushedTo, Push)>> {
        use Recip::*;

        let (sender, recv) = tokio::sync::mpsc::channel(pairs.len());

        let tx = self.transaction().await?;

        futures::stream::iter(pairs)
            .map(Ok::<_, Error>)
            .try_for_each_concurrent(10, {
                let tx = &tx;

                move |(recip, push)| {
                    let mut sender = sender.clone();

                    async move {
                        let Push {
                            msg,
                            tag,
                            timestamp,
                            gid,
                        } = push;

                        let dev = match recip {
                            One(single) => {
                                use SingleRecip::*;

                                match single {
                                    User(uid) => {
                                        one_user(tx, uid, msg, *tag, *timestamp, *gid).await
                                    }

                                    Key(key) => one_key(tx, key, msg, *tag, *timestamp, *gid).await,
                                }
                            }

                            Many(recips) => {
                                use Recips::*;

                                match recips {
                                    Users(uids) => {
                                        many_users(tx, uids, msg, *tag, *timestamp, *gid).await
                                    }

                                    Keys(keys) => {
                                        many_keys(tx, keys, msg, *tag, *timestamp, *gid).await
                                    }
                                }
                            }
                        }?;

                        let push = Push {
                            msg: msg.clone(),
                            tag: *tag,
                            timestamp: *timestamp,
                            gid: *gid,
                        };

                        sender
                            .send((dev, push))
                            .await
                            .expect("If this happens, it's a probably a tokio mpsc bug");

                        Ok(())
                    }
                }
            })
            .await?;

        tx.commit().await?;

        Ok(recv)
    }

    pub async fn get_pending(
        &mut self,
        of: sig::PublicKey,
    ) -> Res<Vec<(Push, i64)>> {
        let stmt = self
            .prepare_typed(sql!("get_pending"), types![BYTEA])
            .await?;

        let rows = self.query(&stmt, params![of.as_ref()]).await?;

        let mut out = Vec::with_capacity(rows.len());

        for row in rows {
            let push_data: &[u8] = row.get("push_data");
            let push_ts: i64 = row.get("push_ts");
            let push_tag: &[u8] = row.get("push_tag");
            let push_id: i64 = row.get("push_id");
            let push_user_id: &str = row.get("push_user_id");
            let push_key: &[u8] = row.get("push_key");

            let push = Push {
                tag: kson::from_slice(push_tag)?,
                msg: Bytes::copy_from_slice(push_data),
                timestamp: Time::from(push_ts),
                gid: GlobalId {
                    uid: UserId::try_from(push_user_id)?,
                    did: sig::PublicKey::from_slice(push_key).ok_or(Error::InvalidKey)?,
                },
            };

            out.push((push, push_id));
        }

        Ok(out)
    }

    pub async fn del_pending<S: Stream<Item = i64> + Send>(
        &mut self,
        of: sig::PublicKey,
        items: S,
    ) -> Res<()> {
        let stmt = self
            .prepare_typed(sql!("expire_pending"), types![BYTEA, INT8])
            .await?;

        items
            .map(Ok::<i64, Error>)
            .try_for_each_concurrent(10, |index| {
                let conn = &self;
                let stmt = &stmt;
                let of = &of;

                async move {
                    conn.execute(stmt, params![of.as_ref(), index]).await?;
                    Ok(())
                }
            })
            .await?;

        self.execute(sql!("del_dangling_pushes"), params![]).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;
