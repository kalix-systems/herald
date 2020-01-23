use anyhow::anyhow;
use anyhow::Error;
use herald_client::HClient as Client;
use herald_common::{
    protocol::{
        auth::register,
        requests::{Request, Response},
    },
    *,
};
use std::thread::Builder as Thread;
use tokio::runtime::Builder as Runtime;
use tokio::sync::mpsc::{channel, unbounded_channel};
pub use tokio::sync::mpsc::{Sender, UnboundedReceiver, UnboundedSender};

pub use registration::RegistrationHandle;
pub use requests::Requester;

mod client;
pub mod registration;
mod requests;
mod setup;

type ResponseHandler = Box<dyn FnMut(Result<Response, Error>) + Send + 'static>;
type HandledReq = (Request, ResponseHandler);

pub type RequestTx = UnboundedSender<HandledReq>;
pub type RequestRx = UnboundedReceiver<HandledReq>;
pub type RegistrationTx = Sender<register::ClientEvent>;

pub trait PushHandler: Send + 'static + Sized {
    fn handle(
        &mut self,
        push: Push,
    );
}

#[inline]
pub fn login<PH, CF>(
    uid: UserId,
    keys: sig::KeyPair,
    server_dns: String,
    server_port: u16,
    push_handler: PH,
    connection_failed: CF,
) -> Result<Requester, Error>
where
    PH: PushHandler,
    CF: FnOnce(Error) + Send + 'static,
{
    let (tx, rx) = unbounded_channel();

    Thread::new().spawn(move || {
        let mut rt = Runtime::new().basic_scheduler().build()?;

        rt.block_on(setup::login_inner(
            uid,
            keys,
            server_dns,
            server_port,
            push_handler,
            connection_failed,
            rx,
        ))?;

        Ok::<(), Error>(())
    })?;

    Ok(tx.into())
}

#[inline]
pub fn register<PH, RH, CF>(
    keys: sig::KeyPair,
    server_dns: String,
    server_port: u16,
    push_handler: PH,
    response_handler: RH,
    connection_failed: CF,
) -> Result<RegistrationHandle, Error>
where
    PH: PushHandler,
    RH: FnMut(register::ServeEvent) + Send + 'static,
    CF: FnOnce(Error) + Send + 'static,
{
    let (registration_tx, registration_rx) = channel(10);
    let (req_tx, req_rx) = unbounded_channel();

    Thread::new().spawn(move || {
        let mut rt = Runtime::new().basic_scheduler().build()?;

        rt.block_on(registration::register_inner(
            registration_rx,
            keys,
            (server_dns, server_port),
            (push_handler, response_handler, connection_failed),
            req_rx,
        ))?;

        Ok::<(), Error>(())
    })?;

    Ok(RegistrationHandle {
        registration_tx,
        request_tx: req_tx,
    })
}

pub fn quit() -> Result<(), Error> {
    let client = client::get().ok_or_else(|| {
        anyhow!("Failed to acquire handle to connection while attempting to close")
    })?;

    client.read().quit()?;

    Ok(())
}
