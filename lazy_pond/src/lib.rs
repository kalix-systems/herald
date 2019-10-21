use parking_lot::Mutex;
use std::ops::{Deref, DerefMut, Drop};

const DEFAULT_SIZE: usize = 32;

pub enum LazyError {
    UnexpectedNone(&'static str, u32),
}

/// Returns an `UnexpectedNone` annotated with the current file and line number.
macro_rules! NE {
    () => {
        LazyError::UnexpectedNone(file!(), line!())
    };
}

pub struct LazyPond<T> {
    connections: Mutex<Vec<T>>,
    max_size: usize,
    ctor: Box<dyn Fn() -> T + Sync + 'static>,
}

pub struct Wrapper<'a, T> {
    pond: &'a LazyPond<T>,
    conn: Option<T>,
}

impl<'a, T> Deref for Wrapper<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // this should not fail
        self.conn.as_ref().expect("Deref failed, unexpected `None`")
    }
}

impl<'a, T> DerefMut for Wrapper<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // this should not fail
        self.conn.as_mut().expect("Deref failed, unexpected `None`")
    }
}

impl<'a, T> Drop for Wrapper<'a, T> {
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

impl<T> LazyPond<T> {
    pub fn new(max_size: Option<usize>, ctor: Box<dyn Fn() -> T + Sync + 'static>) -> LazyPond<T> {
        let connections = Mutex::new(Vec::with_capacity(DEFAULT_SIZE));
        LazyPond {
            connections,
            max_size: max_size.unwrap_or(std::usize::MAX),
            ctor,
        }
    }

    pub fn get(&self) -> Result<Wrapper<T>, LazyError> {
        let conns = &mut self.connections.lock();
        let conn = if !conns.is_empty() {
            conns.pop().ok_or(NE!())?
        } else {
            let ctor = &self.ctor;
            ctor()
        };

        Ok(Wrapper {
            pond: &self,
            conn: Some(conn),
        })
    }
}
