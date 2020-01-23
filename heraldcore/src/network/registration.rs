use super::*;

/// Registers new user on the server.
pub fn begin_registration((dns, port): (String, u16)) -> Result<hn::RegistrationHandle, HErr> {
    kcl::init();

    let kp = sig::KeyPair::gen_new();

    hn::register(
        kp,
        dns,
        port,
        Pushy::default(),
        |_| todo!(),
        |e| push(Notification::ConnectionDown(e)),
    )
}

/// To be call this upon successful registration
pub fn finish_registration(handle: hn::RegistrationHandle) {
    requester::update(handle.into())
}
