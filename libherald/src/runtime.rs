use crate::ret_err;
use crossbeam_channel::{unbounded, Sender};
use heraldcore::{channel_send_err, errors::HErr};
use once_cell::sync::OnceCell;
use std::{pin::Pin, thread};
use tokio::prelude::*;
use tokio::runtime::Runtime;

type Fut = Pin<Box<dyn Future<Output = ()> + Send>>;

static TX: OnceCell<Sender<Fut>> = OnceCell::new();

pub(crate) fn spawn<F: Future<Output = ()> + Send + 'static>(f: F) -> Result<(), HErr> {
    match TX.get() {
        Some(tx) => tx
            .clone()
            .send(Box::pin(f))
            .map_err(|_| channel_send_err!()),
        None => {
            let tx = start()?;
            tx.send(Box::pin(f)).map_err(|_| channel_send_err!())
        }
    }
}

fn start() -> Result<Sender<Fut>, HErr> {
    let (tx, rx) = unbounded();

    // manually drop error
    drop(TX.set((&tx).clone()));

    thread::Builder::new().spawn(move || {
        let rt = ret_err!(Runtime::new());

        for fut in rx.iter() {
            rt.spawn(fut);
        }
    })?;

    Ok(tx)
}
