use super::*;

#[derive(Clone)]
pub struct Requester {
    pub(crate) tx: RequestTx,
}

impl Requester {
    pub fn send<F: FnMut(Result<Response, Error>) + Send + 'static>(
        &mut self,
        req: Request,
        f: F,
    ) -> Result<(), Error> {
        let boxed = Box::new(f);

        // TODO: should this trigger a reconnect?
        // push requests should probably also go in pending
        self.tx
            .send((req, boxed))
            .map_err(|_| anyhow!("Failed to send request"))?;

        Ok(())
    }
}

impl From<RequestTx> for Requester {
    fn from(tx: RequestTx) -> Self {
        Self { tx }
    }
}

impl From<RegistrationHandle> for Requester {
    fn from(handle: RegistrationHandle) -> Self {
        Self {
            tx: handle.request_tx,
        }
    }
}
