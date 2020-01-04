use super::*;

use tokio::sync::{
    mpsc::{channel, Receiver, UnboundedReceiver},
    oneshot,
};

pub(super) async fn login_inner<PH, CF>(
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

pub(super) async fn register_inner<PH, RH, CF>(
    client_rx: Receiver<register::ClientEvent>,
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
        Client::register(client_rx, response_tx, keys, &server_dns, server_port).await?;

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
        if err_rx.await.is_err() {
            connection_failed();
        }
    });

    while let Some(push) = rx.recv().await {
        push_handler(push);
    }

    drop(CLIENT.get().map(|c| c.read().quit()));
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
