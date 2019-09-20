use crate::*;

#[async_trait]
/// Handles protocol messages.
/// For the client, this will tag each message and send it to the server.
/// For the server, this will receive the message, process it, and send the response to the client.
pub trait ProtocolHandler {
    type Error: From<std::io::Error>;
    type From;

    async fn handle_fanout(
        &self,
        from: Self::From,
        fanout: fanout::ToServer,
    ) -> Result<fanout::ServerResponse, Self::Error>;
    async fn handle_pki(
        &self,
        from: Self::From,
        msg: pubkey::ToServer,
    ) -> Result<pubkey::ServerResponse, Self::Error>;
    async fn handle_query(
        &self,
        from: Self::From,
        query: query::ToServer,
    ) -> Result<query::ServerResponse, Self::Error>;
}

#[async_trait]
/// `PushHandler`s must also be able to handle incoming `Push` messages.
pub trait PushHandler: ProtocolHandler {
    async fn handle_push(&self, push: Push) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait Server: ProtocolHandler {
    async fn try_login<S: AsyncWrite + AsyncRead + Unpin>(
        &self,
        pk: sign::PublicKey,
        stream: &mut S,
    ) -> Result<bool, Self::Error>;
    async fn try_register<S: AsyncWrite + AsyncRead + Unpin>(
        &self,
        uid: UserId,
        pk: sign::PublicKey,
        stream: &mut S,
    ) -> Result<bool, Self::Error>;
}
