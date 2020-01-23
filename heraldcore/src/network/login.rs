use super::*;
use handlers::Pushy;

/// Runs login flow
pub fn login() -> Result<(), HErr> {
    let kp = crate::config::keypair()?;
    let uid = crate::config::id()?;
    let (dns, port) = crate::config::home_server()?;

    let handle = hn::login(uid, kp, dns, port, Pushy::default(), |e| {
        push(Notification::ConnectionDown(e))
    })?;

    std::thread::Builder::new().spawn(move || {
        use crypto_store::prelude::*;
        let mut conn = raw_conn().lock();

        let tx = match conn.transaction() {
            Ok(tx) => tx,
            Err(e) => {
                err(e);
                return;
            }
        };

        // TODO
        // - send pending somehow?
        requester::update(handle);
    })?;

    Ok(())
}
