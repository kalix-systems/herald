use super::*;
use crossbeam_channel::{bounded, Receiver, Sender};
use std::ops::{Deref, DerefMut, Drop};

const SIZE: usize = 32;

pub(super) struct Pool {
    tx: Sender<Database>,
    rx: Receiver<Database>,
}

pub(crate) struct Wrapper {
    tx: Sender<Database>,
    conn: Option<Database>,
}

impl Deref for Wrapper {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        // this should not fail
        self.conn.as_ref().expect("Deref failed, unexpected `None`")
    }
}

impl DerefMut for Wrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // this should not fail
        self.conn.as_mut().expect("Deref failed, unexpected `None`")
    }
}

impl Drop for Wrapper {
    fn drop(&mut self) {
        let conn = match self.conn.take() {
            Some(conn) => conn,
            None => {
                // this should never happen
                return;
            }
        };

        drop(self.tx.try_send(conn))
    }
}

impl Pool {
    pub fn new() -> Pool {
        let (tx, rx) = bounded(SIZE);

        Self { tx, rx }
    }

    pub fn get(&self) -> Result<Wrapper, HErr> {
        let conn = match self.rx.try_recv() {
            Ok(db) => db,
            Err(_) => Database::new(DB_PATH.as_str())?,
        };

        Ok(Wrapper {
            tx: self.tx.clone(),
            conn: Some(conn),
        })
    }
}
