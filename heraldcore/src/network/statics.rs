use super::*;

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_SERVER_IP_ADDR: [u8; 4] = [192, 168, 50, 64];

lazy_static! {
    pub(super) static ref SERVER_ADDR: SocketAddr = match &coreconf::CONF.server_addr {
        Some(addr) => addr.parse().unwrap_or_else(|e| {
            eprintln!("Provided address {} is invalid: {}", addr, e);
            std::process::abort();
        }),
        None => SocketAddr::V4(SocketAddrV4::new(
            DEFAULT_SERVER_IP_ADDR.into(),
            DEFAULT_PORT
        )),
    };
}

pub(super) static CAUGHT_UP: AtomicBool = AtomicBool::new(false);
