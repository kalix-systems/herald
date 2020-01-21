use super::*;
use coremacros::exit_err;
use once_cell::sync::OnceCell;

static HOME_SERVER: OnceCell<(String, u16)> = OnceCell::new();

pub(super) fn home_server() -> (String, u16) {
    exit_err!(HOME_SERVER.get_or_try_init(crate::config::home_server)).clone()
}

pub(super) static CAUGHT_UP: AtomicBool = AtomicBool::new(false);
