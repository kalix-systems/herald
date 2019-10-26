use super::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::ops::{Deref, DerefMut, Drop};

pub struct Conn {
    tx: Sender<Client>,
    inner: Option<Client>,
}

impl Deref for Conn {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        // this should not fail
        self.inner
            .as_ref()
            .expect("Deref failed, unexpected `None`")
    }
}

impl DerefMut for Conn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // this should not fail
        self.inner
            .as_mut()
            .expect("Deref failed, unexpected `None`")
    }
}

impl Drop for Conn {
    fn drop(&mut self) {
        let conn = match self.inner.take() {
            Some(conn) => conn,
            None => {
                // this should never happen
                return;
            }
        };

        drop(self.tx.send(conn))
    }
}

async fn get_client() -> Result<Client, PgError> {
    let (client, connection) = tokio_postgres::connect(&DATABASE_URL, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    let connection = connection.map(|r| {
        if let Err(e) = r {
            eprintln!("connection error: {}", e);
        }
    });

    tokio::spawn(connection);

    Ok(client)
}

pub struct Pool {
    tx: Sender<Client>,
    rx: Receiver<Client>,
}

impl Pool {
    pub fn new() -> Pool {
        let (tx, rx) = unbounded();
        Pool { tx, rx }
    }

    pub async fn get(&self) -> Result<Conn, Error> {
        let client: Client = match self.rx.try_recv() {
            Ok(client) => client,
            Err(_) => get_client().await?,
        };

        Ok(Conn {
            tx: self.tx.clone(),
            inner: Some(client),
        })
    }
}
