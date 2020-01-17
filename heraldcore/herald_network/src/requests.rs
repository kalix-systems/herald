use super::*;

#[derive(Clone)]
pub struct Requester {
    pub(crate) tx: RequestTx,
}

impl Requester {
    pub fn send<R: Into<Request>, F: FnMut(Result<Response, Error>) + Send + 'static>(
        &mut self,
        req: R,
        f: F,
    ) -> Result<(), Error> {
        let req = req.into();
        let boxed = Box::new(f);

        // TODO: should this trigger a reconnect?
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
