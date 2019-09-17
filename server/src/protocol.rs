use crate::{prelude::*, store::*};

use dashmap::DashMap;
use tokio::{net::TcpStream, prelude::*, sync::mpsc};
use womp::womp;

pub struct Streams {
    active: DashMap<sig::PublicKey, mpsc::UnboundedSender<MessageToClient>>,
    redis: redis::Client,
}

impl Streams {
    pub fn new<T: redis::IntoConnectionInfo>(redisparams: T) -> Result<Self, Error> {
        sodiumoxide::init().expect("failed to init libsodium");
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
        if let Some(a) = self.active.async_get(to).await {
            let mut sender = a.clone();
            if let Err(m) = sender.try_send(msg) {
                self.new_connection()?.add_pending(to, m.into_inner())?;
            }
        } else {
            self.new_connection()?.add_pending(to, msg)?;
        }
        Ok(())
    }

    pub async fn handle_stream(&'static self, mut stream: TcpStream) -> Result<(), Error> {
        use sodiumoxide::crypto::sign;

        let mut buf = [0u8; sign::PUBLICKEYBYTES];
        stream.read_exact(&mut buf).await?;
        let pk = sign::PublicKey(buf);

        let mut dat = [0u8; 32];
        sodiumoxide::randombytes::randombytes_into(&mut dat);
        stream.write_all(&dat).await?;

        let mut buf = [0u8; sign::SIGNATUREBYTES];
        stream.read_exact(&mut buf).await?;
        let sig = sign::Signature(buf);

        if !sign::verify_detached(&sig, &dat, &pk) {
            stream
                .write_all(b"invalid signature - closing connection")
                .await?;
            return Err(InvalidSig);
        }

        let (sender, receiver) = mpsc::unbounded_channel();
        self.active.insert(pk, sender);
        let (reader, writer) = stream.split();
        self.spawn_msg_sender(pk, writer, receiver);
        self.recv_messages(pk, reader).await?;

        Ok(())
    }

    pub fn spawn_msg_sender<W: AsyncWrite + Unpin + Send + 'static>(
        &'static self,
        pk: sig::PublicKey,
        mut writer: W,
        mut input: mpsc::UnboundedReceiver<MessageToClient>,
    ) {
        tokio::spawn(async move {
            while let Some(msg) = input.recv().await {
                if let Err(e) = send_cbor(&mut writer, &msg).await {
                    eprintln!("failed to write data - assuming connection is closed");
                    eprintln!("error was: {}", e);
                    self.active.remove(&pk);
                    break;
                }
            }

            let mut con = self.new_connection().expect(womp!());
            while let Some(msg) = input.recv().await {
                con.add_pending(pk, msg).expect(womp!());
            }
        });
    }

    // TODO: this
    pub async fn recv_messages<R: AsyncRead + Unpin>(
        &self,
        pk: sig::PublicKey,
        reader: R,
    ) -> Result<(), Error> {
        Ok(())
    }
}
