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

    async fn one_group_exists(
        &mut self,
        cid: &ConversationId,
    ) -> Res<bool> {
        let stmt = self
            .prepare_typed(sql!("group_exists"), types![BYTEA])
            .await?;

        Ok(self.query_one(&stmt, params![cid.as_slice()]).await?.get(0))
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
            Group(ref cid) => self.one_group_exists(cid).await,
            User(ref uid) => self.one_user_exists(uid).await,
            Key(ref key) => self.one_key_exists(key).await,
        }
    }

    async fn many_groups_exist(
        &mut self,
        cids: Vec<ConversationId>,
    ) -> Res<bool> {
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

    async fn many_users_exist(
        &mut self,
        uids: Vec<UserId>,
    ) -> Res<bool> {
        let stmt = self
            .prepare_typed(sql!("user_exists"), types![TEXT])
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

    async fn many_keys_exist(
        &mut self,
        keys: Vec<sig::PublicKey>,
    ) -> Res<bool> {
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

    pub(crate) async fn many_recips_exist(
        &mut self,
        many: Recips,
    ) -> Result<bool, Error> {
        use Recips::*;

        match many {
            Groups(cids) => self.many_groups_exist(cids).await,
            Users(uids) => self.many_users_exist(uids).await,
            Keys(keys) => self.many_keys_exist(keys).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{w, wa};
    use serial_test_derive::serial;
    use std::convert::TryInto;
    use womp::*;

    use crate::tests::get_client;

    #[tokio::test]
    #[serial]
    async fn test_defaults() {
        let mut client = wa!(get_client());

        let kp = sig::KeyPair::gen_new();
        assert!(!wa!(client.one_key_exists(kp.public_key())));
        assert!(!wa!(client.many_keys_exist(vec![*kp.public_key()])));

        let cid = ConversationId::gen_new();
        assert!(!wa!(client.one_group_exists(&cid)));
        assert!(!wa!(client.many_groups_exist(vec![cid])));

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
        let a_init = a_kp.sign(a_uid);
        assert_eq!(wa!(client.new_user(a_init)), register::Res::Success);
        assert!(wa!(client.one_user_exists(&a_uid)));

        let b_uid: UserId = "b".try_into().expect(womp!());
        let b_kp = sig::KeyPair::gen_new();
        let b_init = b_kp.sign(b_uid);
        assert_eq!(wa!(client.new_user(b_init)), register::Res::Success);

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
        assert!(!wa!(client.one_key_exists(a_kp.public_key())));
        let a_init = a_kp.sign(a_uid);
        assert_eq!(wa!(client.new_user(a_init)), register::Res::Success);
        assert!(wa!(client.one_key_exists(a_kp.public_key())));

        let b_uid: UserId = "b".try_into().expect(womp!());
        let b_kp = sig::KeyPair::gen_new();
        assert!(!wa!(client.one_key_exists(b_kp.public_key())));
        let b_init = b_kp.sign(b_uid);
        assert_eq!(wa!(client.new_user(b_init)), register::Res::Success);
        assert!(wa!(
            client.many_keys_exist(vec![*a_kp.public_key(), *b_kp.public_key()])
        ));

        let c_kp = sig::KeyPair::gen_new();
        assert!(!wa!(client.one_key_exists(c_kp.public_key())));
    }

    #[tokio::test]
    #[serial]
    async fn groups() {
        use futures::stream::iter;
        let mut client = wa!(get_client());

        let a_uid: UserId = "a".try_into().expect(womp!());
        let a_kp = sig::KeyPair::gen_new();
        let a_init = a_kp.sign(a_uid);
        wa!(client.new_user(a_init));

        let b_uid: UserId = "b".try_into().expect(womp!());

        let b_kp = sig::KeyPair::gen_new();
        let b_init = b_kp.sign(b_uid);
        wa!(client.new_user(b_init));

        let c_uid: UserId = "c".try_into().expect(womp!());

        let c_kp = sig::KeyPair::gen_new();
        let c_init = c_kp.sign(c_uid);
        wa!(client.new_user(c_init));

        let cid1 = ConversationId::gen_new();

        let uids = vec![a_uid, b_uid, c_uid];

        assert_eq!(
            wa!(client.add_to_group(iter(uids.clone()), cid1)),
            add_to_group::Res::Success
        );

        assert!(wa!(client.one_group_exists(&cid1)));

        let cid2 = ConversationId::gen_new();

        assert_eq!(
            wa!(client.add_to_group(iter(uids.clone()), cid2)),
            add_to_group::Res::Success
        );

        assert!(wa!(client.many_groups_exist(vec![cid1, cid2])));
    }
}
