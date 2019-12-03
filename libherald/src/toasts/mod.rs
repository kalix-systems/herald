#[cfg(not(any(target_os = "android", target_os = "ios", target_os = "windows")))]
/// App name on desktop, used for toasts
const DESKTOP_APP_NAME: &str = "herald";

#[cfg(all(
    target_family = "unix",
    not(any(target_os = "android", target_os = "ios", target_os = "macos"))
))]
mod imp {
    use heraldcore::message::Message;

    /// Displays a new message notification
    // TODO: This should only be called if the user has notifications enabled.
    pub fn new_msg_toast(msg: &Message) {
        use notify_rust::*;

        // Note: If a notification server isn't running, trying to show a notification will
        // block the thread. TODO: Should we inform the user that they need might need to install a
        // notifcation server if one isn't running?
        if get_server_information().is_err() {
            return;
        }

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
    use notify_rust::*;
    use once_cell::sync::OnceCell;

    pub fn new_msg_toast(msg: &Message) {
        if set_application(super::DESKTOP_APP_NAME).is_ok() {
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
