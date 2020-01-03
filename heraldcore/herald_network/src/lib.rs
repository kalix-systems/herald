use anyhow::Error;
use herald_client::HClient as Client;
use herald_common::{protocol::auth::register, *};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::thread::Builder as Thread;
use tokio::runtime::Builder as Runtime;
use tokio::sync::mpsc::channel;
pub use tokio::sync::mpsc::Sender as RegistrationSender;

static CLIENT: OnceCell<RwLock<Client>> = OnceCell::new();

mod setup;

pub fn login<PH, CF>(
    uid: UserId,
    keys: sig::KeyPair,
    server_dns: String,
    server_port: u16,
    push_handler: PH,
    connection_failed: CF,
) -> Result<(), Error>
where
    PH: FnMut(Push) + Send + 'static,
    CF: FnOnce() + Send + 'static,
{
    Thread::new().spawn(move || {
        let mut rt = Runtime::new().basic_scheduler().build()?;

        rt.block_on(setup::login_inner(
            uid,
            keys,
            server_dns,
            server_port,
            push_handler,
            connection_failed,
        ))?;

        Ok::<(), Error>(())
    })?;

    Ok(())
}

pub fn register<PH, RH, CF>(
    keys: sig::KeyPair,
    server_dns: String,
    server_port: u16,
    push_handler: PH,
    response_handler: RH,
    connection_failed: CF,
) -> Result<RegistrationSender<register::ClientEvent>, Error>
where
    PH: FnMut(Push) + Send + 'static,
    RH: FnMut(register::ServeEvent) + Send + 'static,
    CF: FnOnce() + Send + 'static,
{
    let (client_tx, client_rx) = channel(1);

    Thread::new().spawn(move || {
        let mut rt = Runtime::new().basic_scheduler().build()?;

        rt.block_on(setup::register_inner(
            client_rx,
            keys,
            server_dns,
            server_port,
            push_handler,
            response_handler,
            connection_failed,
        ))?;

        Ok::<(), Error>(())
    })?;

    Ok(client_tx)
}
