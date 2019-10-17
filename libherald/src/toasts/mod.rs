/// App name on desktop, used for toasts
const DESKTOP_APP_NAME: &'static str = "heraldqtDesktop";

#[cfg_attr(
    all(
        target_family = "unix",
        not(any(target_os = "android", target_os = "ios", target_os = "macos"))
    ),
    path = "xdg.rs"
)]
#[cfg_attr(target_os = "macos", path = "macos.rs")]
#[cfg_attr(not(target_family = "unix"), path = "other.rs")]
mod imp;

pub(crate) use imp::new_msg_toast;
