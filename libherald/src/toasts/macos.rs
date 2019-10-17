use heraldcore::message::Message;
use notify_rust::*;

fn new_msg_toast(msg: &Message) {
    // TODO: sketchy global state! This should be set
    // somewhere else.
    set_application(crate::DESKTOP_APP_NAME).ok();
    let mut notif = Notification::new();
    notif
        .summary(&format!("New message from {}", msg.author))
        .subtitle("TODO: macOS has subtitles! Do we want them?");

    if let Some(body) = &msg.body {
        notif.body(body.as_str());
    }

    notif.show().ok();
}
