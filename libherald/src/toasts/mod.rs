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
    pub fn new_msg_toast(msg: &Message) {
        use notify_rust::*;

        // Note: If a notification server isn't running, trying to show a notification will
        // block the thread on XDG desktops. TODO: Should we inform the user that they need might need to install a
        // notification server if one isn't running?
        if get_server_information().is_err() {
            return;
        }

        let mut notif = Notification::new();

        notif
            .appname(super::DESKTOP_APP_NAME)
            .summary(&format!("New message from {}", msg.author));

        if let Some(body) = msg.text() {
            notif.body(body);
        }

        drop(
            notif
                .hint(NotificationHint::Category("im.received".to_owned()))
                .show(),
        );
    }
}

#[cfg(target_os = "macos")]
mod imp {
    use heraldcore::message::Message;
    use notify_rust::*;
    use once_cell::sync::OnceCell;

    static IS_SET: OnceCell<bool> = OnceCell::new();

    fn setup() -> bool {
        *IS_SET
            .get_or_try_init(|| {
                let bundle = get_bundle_identifier_or_default(super::DESKTOP_APP_NAME);
                drop(set_application(&bundle));
            })
            .unwrap_or(&false)
    }

    pub fn new_msg_toast(msg: &Message) {
        if !setup() {
            return;
        }

        let mut notif = Notification::new();

        notif.summary(&format!("New message from {}", msg.author));

        if let Some(body) = msg.text() {
            notif.body(body);
        }

        drop(notif.show());
    }
}

#[cfg(any(target_os = "android", target_os = "ios", target_os = "windows"))]
mod imp {
    use heraldcore::message::Message;

    /// No-op
    pub fn new_msg_toast(_: &Message) {}
}

pub(crate) use imp::new_msg_toast;
