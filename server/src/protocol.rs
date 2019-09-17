use crate::{prelude::*, store::*};

use dashmap::DashMap;
use tokio::prelude::*;
use tokio::sync::mpsc;

pub struct Streams {
    active: DashMap<sig::PublicKey, mpsc::Sender<MessageToClient>>,
    redis: redis::Client,
}

impl Streams {
    pub fn new<T: redis::IntoConnectionInfo>(redisparams: T) -> Result<Self, Error> {
        Ok(Streams {
            active: DashMap::default(),
            redis: redis::Client::open(redisparams)?,
        })
    }

    fn new_connection(&self) -> Result<redis::Connection, Error> {
        Ok(self.redis.get_connection()?)
    }

    fn with_db_tx<K, T, F>(&self, keys: &[K], f: F) -> Result<T, Error>
    where
        K: redis::ToRedisArgs,
        T: redis::FromRedisValue,
        F: FnMut(&mut redis::Connection, &mut redis::Pipeline) -> redis::RedisResult<Option<T>>,
    {
        Ok(redis::transaction(&mut self.new_connection()?, keys, f)?)
    }

    pub async fn send_message(
        &self,
        to: sig::PublicKey,
        msg: MessageToClient,
    ) -> Result<(), Error> {
        let mut pending = true;
        if let Some(a) = self.active.async_get(to).await {
            pending = false;
            let mut sender = a.clone();
            if let Err(_) = sender.send(msg.clone()).await {
                pending = true;
            }
        }
        if pending {
            self.new_connection()?.add_pending(to, msg)?;
        }
        Ok(())
    }
}
