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

#[derive(Clone)]
pub struct RegistrationHandle {
    pub(crate) registration_tx: RegistrationTx,
    pub(crate) request_tx: RequestTx,
}

#[derive(Debug, Clone, Copy)]
#[must_use]
pub enum Outcome {
    Wait,
    Sending,
}
impl RegistrationHandle {
    pub fn check_user_id(
        &mut self,
        uid: UserId,
    ) -> Result<Outcome, Error> {
        if let Err(e) = self
            .registration_tx
            .try_send(register::ClientEvent::Check(uid))
        {
            {
                use tokio::sync::mpsc::error::TrySendError as E;
                match e {
                    E::Full(_) => return Ok(Outcome::Wait),
                    E::Closed(_) => anyhow!("Failed to check user id: {}", uid),
                }
            };
        }

        Ok(Outcome::Sending)
    }

    pub fn claim_user_id(
        &mut self,
        uid: Signed<UserId>,
    ) -> Result<Outcome, Error> {
        if let Err(e) = self
            .registration_tx
            .try_send(register::ClientEvent::Claim(uid))
        {
            {
                use tokio::sync::mpsc::error::TrySendError as E;
                match e {
                    E::Full(_) => return Ok(Outcome::Wait),
                    E::Closed(_) => anyhow!("Failed to claim user id: {}", uid.data()),
                }
            };
        }

        Ok(Outcome::Sending)
    }
}
