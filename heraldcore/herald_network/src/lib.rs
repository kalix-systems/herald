use anyhow::Error;
use herald_client::HClient as Client;
use herald_common::{protocol::auth::register, *};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::thread::Builder as Thread;
use tokio::runtime::Builder as Runtime;
pub use tokio::sync::mpsc::Sender as RegistrationSender;
use tokio::sync::mpsc::{channel, Receiver};

static CLIENT: OnceCell<RwLock<Client>> = OnceCell::new();

pub fn login<F>(
    uid: UserId,
    keys: sig::KeyPair,
    server_dns: String,
    server_port: u16,
    push_handler: F,
) -> Result<(), Error>
where
    F: FnMut(Push) + Send + 'static,
{
    Thread::new().spawn(move || {
        let mut rt = Runtime::new().basic_scheduler().build()?;

        rt.block_on(login_inner(
            uid,
            keys,
            server_dns,
            server_port,
            push_handler,
        ))?;

        Ok::<(), Error>(())
    })?;

    Ok(())
}

pub fn register<PH, RH>(
    keys: sig::KeyPair,
    server_dns: String,
    server_port: u16,
    push_handler: PH,
    response_handler: RH,
) -> Result<RegistrationSender<register::ClientEvent>, Error>
where
    PH: FnMut(Push) + Send + 'static,
    RH: FnMut(register::ServeEvent) + Send + 'static,
{
    let (client_tx, client_rx) = channel(1);

    Thread::new().spawn(move || {
        let mut rt = Runtime::new().basic_scheduler().build()?;

        rt.block_on(register_inner(
            client_rx,
            keys,
            server_dns,
            server_port,
            push_handler,
            response_handler,
        ))?;

        Ok::<(), Error>(())
    })?;

    Ok(client_tx)
}

async fn login_inner<F>(
    uid: UserId,
    keys: sig::KeyPair,
    server_dns: String,
    server_port: u16,
    mut push_handler: F,
) -> Result<(), Error>
where
    F: FnMut(Push) + Send + 'static,
{
    let (client, mut rx) = Client::login(uid, keys, &server_dns, server_port).await?;

    update_client(client);

    while let Some(push) = rx.recv().await {
        push_handler(push);
    }

    Ok::<(), Error>(())
}

async fn register_inner<PH, RH>(
    uid_rx: Receiver<register::ClientEvent>,
    keys: sig::KeyPair,
    server_dns: String,
    server_port: u16,
    mut push_handler: PH,
    mut response_handler: RH,
) -> Result<(), Error>
where
    PH: FnMut(Push) + Send + 'static,
    RH: FnMut(register::ServeEvent) + Send + 'static,
{
    let (response_tx, mut response_rx) = channel(1);

    let (client, mut rx) =
        Client::register(uid_rx, response_tx, keys, &server_dns, server_port).await?;

    while let Some(response) = response_rx.recv().await {
        response_handler(response);
    }

    update_client(client);

    while let Some(push) = rx.recv().await {
        push_handler(push);
    }

    Ok::<(), Error>(())
}

fn update_client(client: Client) {
    // try to set the client
    if let Err(client) = CLIENT.set(RwLock::new(client)) {
        // if it fails, this means the client is already set, so we should replace it
        if let Some(lock) = CLIENT.get() {
            let mut locked = lock.write();
            *locked = client.into_inner();
        }
    }
}
