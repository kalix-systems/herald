#[cfg(not(any(target_os = "android", target_os = "ios")))]
/// App name on desktop, used for toasts
const DESKTOP_APP_NAME: &'static str = "heraldqtDesktop";

#[cfg(all(
    target_family = "unix",
    not(any(target_os = "android", target_os = "ios", target_os = "macos"))
))]
mod imp {
    use heraldcore::message::Message;

    /// Displays a new message notification
    pub fn new_msg_toast(msg: &Message) {
        use notify_rust::*;

        let mut notif = Notification::new();
        notif
            .appname(super::DESKTOP_APP_NAME)
            .summary(&format!("New message from {}", msg.author));

        if let Some(body) = &msg.body {
            notif.body(body.as_str());
        }

        notif
            .hint(NotificationHint::Category("im.received".to_owned()))
            .show()
            .ok();
    }
}

#[cfg(target_os = "macos")]
mod imp {
    use heraldcore::message::Message;
    use lazy_static::*;
    use notify_rust::*;

    lazy_static! {
        static ref SET_APP_RES: Option<()> = set_application(super::DESKTOP_APP_NAME).ok();
    }

    pub fn new_msg_toast(msg: &Message) {
        if SET_APP_RES.is_some() {
            let mut notif = Notification::new();
            notif
                .summary(&format!("New message from {}", msg.author))
                .subtitle("TODO: macOS has subtitles! Do we want them?");

            if let Some(body) = &msg.body {
                notif.body(body.as_str());
            }

            notif.show().ok();
        }
    }
}

#[cfg(any(target_os = "android", target_os = "ios", target_os = "windows"))]
mod imp {
    use heraldcore::message::Message;

    /// No-op
    pub fn new_msg_toast(_: &Message) {}
}

pub(crate) use imp::new_msg_toast;
