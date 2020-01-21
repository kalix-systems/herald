use super::*;
use crate::client;
use tokio::sync::{mpsc::UnboundedReceiver, oneshot};

pub(super) async fn login_inner<PH, CF>(
    uid: UserId,
    keys: sig::KeyPair,
    server_dns: String,
    server_port: u16,
    push_handler: PH,
    connection_failed: CF,
    request_rx: RequestRx,
) -> Result<(), Error>
where
    PH: FnMut(Push) + Send + 'static,
    CF: FnOnce(Error) + Send + 'static,
{
    let (client, rx, err_rx) = Client::login(uid, keys, &server_dns, server_port).await?;

    client::update(client);
    handle_events(push_handler, connection_failed, rx, err_rx, request_rx).await;

    Ok(())
}

pub(crate) async fn handle_events<PH, CF>(
    mut push_handler: PH,
    connection_failed: CF,
    mut rx: UnboundedReceiver<Push>,
    err_rx: oneshot::Receiver<Error>,
    mut req_rx: RequestRx,
) where
    PH: FnMut(Push) + Send + 'static,
    CF: FnOnce(Error) + Send + 'static,
{
    tokio::spawn(handle_connection_failure(connection_failed, err_rx));
    tokio::spawn(async move {
        while let Some((request, mut f)) = req_rx.recv().await {
            if let Some(client) = client::get() {
                let response_res = client.read().req(request);

                match response_res {
                    Ok(rx) => match rx.await {
                        Ok(response) => f(Ok(response)),
                        Err(e) => f(Err(e.into())),
                    },
                    Err(e) => f(Err(e)),
                }
            }
        }
    });

    while let Some(push) = rx.recv().await {
        push_handler(push);
    }

    drop(crate::quit());
}

async fn handle_connection_failure<CF>(
    connection_failed: CF,
    err_rx: oneshot::Receiver<Error>,
) where
    CF: FnOnce(Error) + Send + 'static,
{
    if let Ok(e) = err_rx.await {
        connection_failed(e);
    }

    drop(crate::quit());
}
