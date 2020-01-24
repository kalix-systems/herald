use super::*;
use coremacros::w;
use handlers::Pushy;

/// Runs login flow
pub fn login() -> Result<(), HErr> {
    let (uid, kp) = w!(crate::config::id_kp());
    let (dns, port) = w!(crate::config::home_server());

    let handle = hn::login(uid, kp, dns, port, Pushy::default(), |e| {
        push(Notification::ConnectionDown(e))
    })?;

    std::thread::Builder::new().spawn(move || {
        use crypto_store::prelude::*;
        let mut conn = raw_conn().lock();

        let tx = w!(conn.transaction());

        // TODO
        // - send pending somehow?
        requester::update(handle);

        Ok::<(), HErr>(())
    })?;

    Ok(())
}
