use super::*;
use once_cell::sync::OnceCell;

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_SERVER_IP_ADDR: [u8; 4] = [127, 0, 0, 1];

static HOME_SERVER: OnceCell<SocketAddr> = OnceCell::new();
static DEFAULT_SERVER: OnceCell<SocketAddr> = OnceCell::new();

pub(super) fn home_server() -> &'static SocketAddr {
    match HOME_SERVER.get_or_try_init(|| crate::config::home_server()) {
        Ok(addr) => addr,
        Err(_) => default_server(),
    }
}

pub(crate) fn default_server() -> &'static SocketAddr {
    DEFAULT_SERVER
        .get_or_init(|| SocketAddr::new(DEFAULT_SERVER_IP_ADDR.into(), DEFAULT_PORT.into()))
}

pub(super) static CAUGHT_UP: AtomicBool = AtomicBool::new(false);
