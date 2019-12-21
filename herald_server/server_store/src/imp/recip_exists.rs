use super::*;

macro_rules! sql {
    ($path: literal) => {
        include_str!(concat!("../sql/", $path, ".sql"))
    };
}

impl Conn {
    pub(crate) async fn one_recip_exists(
        &mut self,
        single: SingleRecip,
    ) -> Result<bool, Error> {
        use SingleRecip::*;
        match single {
            Group(cid) => {
                let stmt = self
                    .prepare_typed(sql!("group_exists"), types![BYTEA])
                    .await?;

                Ok(self.query_one(&stmt, params![cid.as_slice()]).await?.get(0))
            }

            User(uid) => {
                let stmt = self
                    .prepare_typed(sql!("user_exists"), types![TEXT])
                    .await?;

                Ok(self.query_one(&stmt, params![uid.as_str()]).await?.get(0))
            }

            Key(key) => {
                let stmt = self
                    .prepare_typed(sql!("device_exists"), types![BYTEA])
                    .await?;

                let row = self.query_one(&stmt, params![key.as_ref()]).await?;

                Ok(row.get(0))
            }
        }
    }

    pub(crate) async fn many_recip_exists(
        &mut self,
        many: Recips,
    ) -> Result<bool, Error> {
        use Recips::*;

        match many {
            Groups(cids) => {
                let stmt = self
                    .prepare_typed(sql!("group_exists"), types![BYTEA])
                    .await?;

                for cid in cids {
                    let cid_slice = cid.as_slice();
                    let row = self.query_one(&stmt, params![cid_slice]).await?;

                    if !row.get::<_, bool>(0) {
                        return Ok(false);
                    }
                }

                Ok(true)
            }

            Users(uids) => {
                let stmt = self
                    .prepare_typed(sql!("user_exists"), types![BYTEA])
                    .await?;

                for uid in uids {
                    let uid_str = uid.as_str();
                    let row = self.query_one(&stmt, params![uid_str]).await?;

                    if !row.get::<_, bool>(0) {
                        return Ok(false);
                    }
                }

                Ok(true)
            }

            Keys(keys) => {
                let stmt = self
                    .prepare_typed(sql!("device_exists"), types![BYTEA])
                    .await?;

                for key in keys {
                    let key_slice = key.as_ref();

                    let row = self.query_one(&stmt, params![key_slice]).await?;

                    if !row.get::<_, bool>(0) {
                        return Ok(false);
                    }
                }

                Ok(true)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test_derive::serial;
    use std::convert::TryInto;
    use womp::*;

    macro_rules! w {
        ($maybe_val: expr) => {
            $maybe_val.expect(womp!())
        };
    }

    macro_rules! wa {
        ($maybe_fut: expr) => {
            w!($maybe_fut.await)
        };
    }

    use crate::tests::get_client;

    #[tokio::test]
    #[serial]
    async fn device_exists() {
        let mut client = wa!(get_client());
        wa!(client.reset_all());

        //let kp = sig::KeyPair::gen_new();
        //let user_id = "Hello".try_into().expect(womp!());

        //let signed_pk = kp.sign(*kp.public_key());
        //assert!(!wa!(client.device_exists(kp.public_key())));

        //wa!(client.register_user(user_id, signed_pk));

        //assert!(wa!(client.device_exists(kp.public_key())));
    }
}
