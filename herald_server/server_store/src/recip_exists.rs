use super::*;

impl Conn {
    pub async fn recip_exists(
        &mut self,
        recip: Recip,
    ) -> Res<bool> {
        use Recip::*;

        match recip {
            One(single) => self.one_recip_exists(single).await,
            Many(recips) => self.many_recips_exist(recips).await,
        }
    }

    async fn one_user_exists(
        &mut self,
        uid: &UserId,
    ) -> Res<bool> {
        let stmt = self
            .prepare_typed(sql!("user_exists"), types![TEXT])
            .await?;

        Ok(self.query_one(&stmt, params![uid.as_str()]).await?.get(0))
    }

    async fn one_key_exists(
        &mut self,
        key: &sig::PublicKey,
    ) -> Res<bool> {
        let stmt = self
            .prepare_typed(sql!("device_exists"), types![BYTEA])
            .await?;

        let row = self.query_one(&stmt, params![key.as_ref()]).await?;

        Ok(row.get(0))
    }

    pub(crate) async fn one_recip_exists(
        &mut self,
        single: SingleRecip,
    ) -> Res<bool> {
        use SingleRecip::*;
        match single {
            User(ref uid) => self.one_user_exists(uid).await,
            Key(ref key) => self.one_key_exists(key).await,
        }
    }

    async fn many_users_exist(
        &mut self,
        uids: Vec<UserId>,
    ) -> Res<bool> {
        let stmt = self
            .prepare_typed(sql!("user_exists"), types![TEXT])
            .await?;

        // TODO: process concurrently?
        for uid in uids {
            let uid_str = uid.as_str();
            let row = self.query_one(&stmt, params![uid_str]).await?;

            if !row.get::<_, bool>(0) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    async fn many_keys_exist(
        &mut self,
        keys: Vec<sig::PublicKey>,
    ) -> Res<bool> {
        let stmt = self
            .prepare_typed(sql!("device_exists"), types![BYTEA])
            .await?;

        // TODO: process concurrently?
        for key in keys {
            let key_slice = key.as_ref();

            let row = self.query_one(&stmt, params![key_slice]).await?;

            if !row.get::<_, bool>(0) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub(crate) async fn many_recips_exist(
        &mut self,
        many: Recips,
    ) -> Res<bool> {
        use Recips::*;

        match many {
            Users(uids) => self.many_users_exist(uids).await,
            Keys(keys) => self.many_keys_exist(keys).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{w, wa};
    use protocol::auth::register::ServeEvent;
    use serial_test_derive::serial;
    use sig::sign_ser as sign;
    use std::convert::TryInto;
    use womp::*;

    use crate::tests::get_client;

    #[tokio::test]
    #[serial]
    async fn test_defaults() {
        let mut client = wa!(get_client());

        let kp = sig::KeyPair::gen_new();
        assert!(!wa!(client.one_key_exists(kp.public())));
        assert!(!wa!(client.many_keys_exist(vec![*kp.public()])));

        let uid = "a".try_into().expect(womp!());
        assert!(!wa!(client.one_user_exists(&uid)));
        assert!(!wa!(client.many_users_exist(vec![uid])));
    }

    #[tokio::test]
    #[serial]
    async fn users() {
        let mut client = wa!(get_client());

        let a_uid: UserId = "a".try_into().expect(womp!());
        let a_kp = sig::KeyPair::gen_new();
        let a_init = sign(&a_kp, a_uid);
        assert_eq!(wa!(client.new_user(a_init)), ServeEvent::Success);
        assert!(wa!(client.one_user_exists(&a_uid)));

        let b_uid: UserId = "b".try_into().expect(womp!());
        let b_kp = sig::KeyPair::gen_new();
        let b_init = sign(&b_kp, b_uid);
        assert_eq!(wa!(client.new_user(b_init)), ServeEvent::Success);

        assert!(wa!(client.many_users_exist(vec![a_uid, b_uid])));

        let c_uid: UserId = "c".try_into().expect(womp!());
        assert!(!wa!(client.one_user_exists(&c_uid)));
    }

    #[tokio::test]
    #[serial]
    async fn keys() {
        let mut client = wa!(get_client());

        let a_uid: UserId = "a".try_into().expect(womp!());
        let a_kp = sig::KeyPair::gen_new();
        assert!(!wa!(client.one_key_exists(a_kp.public())));

        let a_init = sign(&a_kp, a_uid);
        assert_eq!(wa!(client.new_user(a_init)), ServeEvent::Success);
        assert!(wa!(client.one_key_exists(a_kp.public())));

        let b_uid: UserId = "b".try_into().expect(womp!());
        let b_kp = sig::KeyPair::gen_new();
        assert!(!wa!(client.one_key_exists(b_kp.public())));
        let b_init = sign(&b_kp, b_uid);
        assert_eq!(wa!(client.new_user(b_init)), ServeEvent::Success);
        assert!(wa!(
            client.many_keys_exist(vec![*a_kp.public(), *b_kp.public()])
        ));

        let c_kp = sig::KeyPair::gen_new();
        assert!(!wa!(client.one_key_exists(c_kp.public())));
    }
}
