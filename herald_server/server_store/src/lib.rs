#![recursion_limit = "256"]

use futures::{lock::Mutex, try_join, FutureExt, Stream, StreamExt, TryStreamExt};
use herald_common::*;
use server_errors::Error;
use std::convert::TryFrom;

use tokio_postgres::{
    types::{ToSql, Type},
    Client, Error as PgError, NoTls,
};

type Res<T> = std::result::Result<T, Error>;

mod macros;
mod pending;
mod pool;
mod prekeys;
mod recip_exists;
mod sigchain;
pub use pending::PushedTo;
pub use pool::*;
pub use prekeys::{PrekeyReplace, TaggedPrekey};

impl Conn {
    pub async fn user_of(
        &mut self,
        key: sig::PublicKey,
    ) -> Res<Option<UserId>> {
        let stmt = self.prepare_typed(sql!("user_of"), types![BYTEA]).await?;

        let rows = self.query(&stmt, params![key.as_ref()]).await?;

        let first = match rows.into_iter().next() {
            Some(first) => first,
            None => return Ok(None),
        };

        Ok(UserId::try_from(first.get::<_, &str>("user_id")).ok())
    }

    pub async fn new_user(
        &mut self,
        init: Signed<UserId>,
    ) -> Res<protocol::auth::RegisterResponse> {
        use protocol::auth::RegisterResponse;

        let (user_id, meta) = init.split();

        let tx = self.transaction().await?;

        let exists_stmt = tx.prepare_typed(sql!("user_exists"), types![TEXT]).await?;

        if tx
            .query_one(&exists_stmt, params![user_id.as_str()])
            .await?
            .get::<_, bool>(0)
        {
            return Ok(RegisterResponse::Taken);
        }

        let add_key_stmt = tx
            .prepare_typed(sql!("add_key"), types![BYTEA, BYTEA, INT8])
            .await?;

        tx.execute(
            &add_key_stmt,
            params![
                meta.signed_by().as_ref(),
                meta.sig().as_ref(),
                meta.timestamp().as_i64(),
            ],
        )
        .await?;

        let add_user_key_stmt = tx
            .prepare_typed(sql!("add_user_key"), types![TEXT, BYTEA])
            .await?;

        tx.execute(
            &add_user_key_stmt,
            params![user_id.as_str(), meta.signed_by().as_ref()],
        )
        .await?;
        tx.commit().await?;

        Ok(RegisterResponse::Success)
    }

    pub async fn key_is_valid(
        &mut self,
        key: sig::PublicKey,
    ) -> Res<bool> {
        let stmt = self
            .prepare_typed(sql!("key_is_valid"), types![BYTEA])
            .await?;

        Ok(self
            .query_one(&stmt, params![key.as_ref()])
            .await?
            .get::<_, bool>(0))
    }

    pub async fn key_is_valid_for_user(
        &mut self,
        key: &sig::PublicKey,
        user: &UserId,
    ) -> Res<bool> {
        let stmt = self
            .prepare_typed(sql!("key_is_valid_for_user"), types![TEXT, BYTEA])
            .await?;

        Ok(self
            .query_one(&stmt, params![user.as_str(), key.as_ref()])
            .await?
            .get::<_, bool>(0))
    }
}

impl Conn {
    pub async fn setup(&mut self) -> Res<()> {
        // create
        self.batch_execute(include_str!("../schema/up.sql")).await?;
        Ok(())
    }

    pub async fn reset_all(&mut self) -> Res<()> {
        let tx = self.transaction().await?;

        // drop
        tx.batch_execute(include_str!("../schema/down.sql")).await?;

        // create
        tx.batch_execute(include_str!("../schema/up.sql")).await?;
        tx.commit().await?;
        Ok(())
    }
}

// helper function for using query_raw methods
fn slice_iter<'a>(
    s: &'a [&'a (dyn ToSql + Sync)]
) -> impl ExactSizeIterator<Item = &'a dyn ToSql> + 'a {
    s.iter().map(|s| *s as _)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use protocol::auth::RegisterResponse;
    use serial_test_derive::serial;
    use sig::sign_ser as sign;
    use std::convert::TryInto;
    use womp::*;

    #[macro_export]
    macro_rules! w {
        ($maybe_val: expr) => {
            $maybe_val.expect(womp!())
        };
    }

    #[macro_export]
    macro_rules! wa {
        ($maybe_fut: expr) => {
            w!($maybe_fut.await)
        };
    }

    pub(crate) async fn get_client() -> Result<Conn, Error> {
        let pool = Pool::new();
        let mut client = pool.get().await?;
        client.reset_all().await?;
        Ok(client)
    }

    #[tokio::test]
    #[serial]
    async fn new_user_and_user_of() {
        let mut client = wa!(get_client());

        let uid: UserId = w!("a".try_into());
        let kp = sig::KeyPair::gen_new();
        let pk = *kp.public();

        let init = sign(&kp, uid);

        assert!(wa!(client.user_of(pk)).is_none());

        assert_eq!(wa!(client.new_user(init)), RegisterResponse::Success);

        assert_eq!(wa!(client.user_of(pk)), Some(uid));
    }

    #[tokio::test]
    #[serial]
    async fn key_is_valid() {
        let mut client = wa!(get_client());

        let uid: UserId = w!("a".try_into());
        let kp = sig::KeyPair::gen_new();
        let pk = *kp.public();

        let init = sign(&kp, uid);

        assert!(!wa!(client.key_is_valid(pk)));

        assert_eq!(wa!(client.new_user(init)), RegisterResponse::Success);

        assert!(wa!(client.key_is_valid(pk)));
    }

    #[tokio::test]
    #[serial]
    async fn key_is_valid_for_user() {
        let mut client = wa!(get_client());

        let uid: UserId = w!("a".try_into());

        let other_uid: UserId = w!("b".try_into());

        let kp = sig::KeyPair::gen_new();
        let pk = *kp.public();

        let init = sign(&kp, uid);

        assert!(!wa!(client.key_is_valid_for_user(&pk, &uid)));

        assert_eq!(wa!(client.new_user(init)), RegisterResponse::Success);

        assert!(!wa!(client.key_is_valid_for_user(&pk, &other_uid)));
        assert!(wa!(client.key_is_valid_for_user(&pk, &uid)));
    }
}
