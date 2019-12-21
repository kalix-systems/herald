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
