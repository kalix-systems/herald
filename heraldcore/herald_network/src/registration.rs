use super::*;
use tokio::sync::mpsc::{channel, Receiver};

pub(super) async fn register_inner<PH, RH, CF>(
    client_rx: Receiver<register::ClientEvent>,
    keys: sig::KeyPair,
    (server_dns, server_port): (String, u16),
    (push_handler, mut response_handler, connection_failed): (PH, RH, CF),
    request_rx: RequestRx,
) -> Result<(), Error>
where
    PH: FnMut(Push) + Send + 'static,
    RH: FnMut(register::ServeEvent) + Send + 'static,
    CF: FnOnce(Error) + Send + 'static,
{
    let (response_tx, mut response_rx) = channel(1);

    let (client, rx, err_rx) =
        Client::register(client_rx, response_tx, keys, &server_dns, server_port).await?;

    while let Some(response) = response_rx.recv().await {
        response_handler(response);
    }

    client::update(client);
    crate::setup::handle_events(push_handler, connection_failed, rx, err_rx, request_rx).await;

    Ok::<(), Error>(())
}
