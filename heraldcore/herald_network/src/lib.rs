use anyhow::Error;
use herald_client::HClient as Client;
use herald_common::{protocol::auth::register, *};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::thread::Builder as Thread;
use tokio::runtime::Builder as Runtime;
pub use tokio::sync::mpsc::Sender as RegistrationSender;
use tokio::sync::{
    mpsc::{channel, Receiver, UnboundedReceiver},
    oneshot,
};

static CLIENT: OnceCell<RwLock<Client>> = OnceCell::new();

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

        rt.block_on(login_inner(
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

        rt.block_on(register_inner(
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

async fn login_inner<PH, CF>(
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
    let (client, rx, err_rx) = Client::login(uid, keys, &server_dns, server_port).await?;

    update_client(client);
    handle_events(push_handler, connection_failed, rx, err_rx).await;

    Ok::<(), Error>(())
}

async fn register_inner<PH, RH, CF>(
    uid_rx: Receiver<register::ClientEvent>,
    keys: sig::KeyPair,
    server_dns: String,
    server_port: u16,
    push_handler: PH,
    mut response_handler: RH,
    connection_failed: CF,
) -> Result<(), Error>
where
    PH: FnMut(Push) + Send + 'static,
    RH: FnMut(register::ServeEvent) + Send + 'static,
    CF: FnOnce() + Send + 'static,
{
    let (response_tx, mut response_rx) = channel(1);

    let (client, rx, err_rx) =
        Client::register(uid_rx, response_tx, keys, &server_dns, server_port).await?;

    while let Some(response) = response_rx.recv().await {
        response_handler(response);
    }

    update_client(client);

    handle_events(push_handler, connection_failed, rx, err_rx).await;

    Ok::<(), Error>(())
}

async fn handle_events<PH, CF>(
    mut push_handler: PH,
    connection_failed: CF,
    mut rx: UnboundedReceiver<Push>,
    err_rx: oneshot::Receiver<Error>,
) where
    PH: FnMut(Push) + Send + 'static,
    CF: FnOnce() + Send + 'static,
{
    tokio::spawn(async move {
        if let Err(_) = err_rx.await {
            connection_failed();
        }
    });

    while let Some(push) = rx.recv().await {
        push_handler(push);
    }
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
