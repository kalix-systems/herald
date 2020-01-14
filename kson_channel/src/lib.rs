use kson::prelude::*;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "async")]
pub mod asynchronous;
pub mod error;
pub use error::FramedError;
pub mod sync;

// use tokio::{prelude::*, time::*};

// mod packets;
// use packets::*;

pub struct Framed<S> {
    inner: S,
    // dur: Duration,
    // packet_size: usize,
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
    pub fn new(
        inner: S,
        // dur: Duration,
        // packet_size: usize,
    ) -> Self {
        Framed {
            inner,
            // dur,
            // packet_size,
        }
    }

    pub fn into_inner(self) -> S {
        self.inner
    }
}
