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
        pki: pubkey::ToServer,
    ) -> Result<pubkey::ServerResponse, Self::Error>;
    async fn handle_query(
        &self,
        from: Self::From,
        query: query::ToServer,
    ) -> Result<query::ServerResponse, Self::Error>;

    async fn handle_message_to_server(
        &self,
        from: Self::From,
        msg: MessageToServer,
    ) -> Result<Response, Self::Error>
    where
        Self::From: Send,
        Self::Error: Send,
    {
        use MessageToServer::*;
        Ok(match msg {
            Fanout(f) => Response::Fanout(self.handle_fanout(from, f).await?),
            PKI(p) => Response::PKI(self.handle_pki(from, p).await?),
            Query(q) => Response::Query(self.handle_query(from, q).await?),
        })
    }
}

#[async_trait]
/// `PushHandler`s must also be able to handle incoming `Push` messages.
pub trait PushHandler: ProtocolHandler {
    async fn handle_push(&self, push: Push) -> Result<(), Self::Error>;
}
