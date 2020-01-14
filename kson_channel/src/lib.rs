use kson::prelude::*;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "async")]
pub mod asynchronous;
pub mod error;
pub use error::FramedError;
pub mod sync;

pub struct Framed<S> {
    inner: S,
}

impl<S> Deref for Framed<S> {
    type Target = S;
    fn deref(&self) -> &S {
        &self.inner
    }
}

impl<S> DerefMut for Framed<S> {
    fn deref_mut(&mut self) -> &mut S {
        &mut self.inner
    }
}

impl<S> Framed<S> {
    pub fn new(inner: S) -> Self {
        Framed { inner }
    }

    pub fn into_inner(self) -> S {
        self.inner
    }
}
