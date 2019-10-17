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
