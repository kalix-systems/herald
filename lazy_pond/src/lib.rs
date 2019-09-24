use parking_lot::Mutex;
use std::ops::{Deref, DerefMut, Drop};

const DEFAULT_SIZE: usize = 32;

pub enum LazyError {
    UnexpectedNone,
}

pub struct LazyPond<T: Default> {
    connections: Mutex<Vec<T>>,
    max_size: usize,
}

pub struct Wrapper<'a, T: Default> {
    pond: &'a LazyPond<T>,
    conn: Option<T>,
}

impl<'a, T: Default> Deref for Wrapper<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // this should not fail
        self.conn.as_ref().expect("Deref failed, unexpected `None`")
    }
}

impl<'a, T: Default> DerefMut for Wrapper<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // this should not fail
        self.conn.as_mut().expect("Deref failed, unexpected `None`")
    }
}

impl<'a, T: Default> Drop for Wrapper<'a, T> {
    fn drop(&mut self) {
        let conn = match std::mem::replace(&mut self.conn, None) {
            Some(conn) => conn,
            None => {
                // this should never happen
                return;
            }
        };

        let connections = &mut self.pond.connections.lock();

        if connections.len() <= self.pond.max_size {
            connections.push(conn);
        }
    }
}

impl<T: Default> LazyPond<T> {
    pub fn new(max_size: Option<usize>) -> LazyPond<T> {
        let connections = Mutex::new(Vec::with_capacity(DEFAULT_SIZE));
        LazyPond {
            connections,
            max_size: max_size.unwrap_or(std::usize::MAX),
        }
    }

    pub fn get(&self) -> Result<Wrapper<T>, LazyError> {
        let conns = &mut self.connections.lock();
        let conn = if !conns.is_empty() {
            conns.pop().ok_or(LazyError::UnexpectedNone)?
        } else {
            T::default()
        };

        Ok(Wrapper {
            pond: &self,
            conn: Some(conn),
        })
    }
}
