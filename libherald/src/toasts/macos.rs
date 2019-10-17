use heraldcore::message::Message;
use lazy_static::*;
use notify_rust::*;

lazy_static! {
    static ref SET_APP_RES: Option<()> = set_application(super::DESKTOP_APP_NAME).ok();
}

fn new_msg_toast(msg: &Message) {
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
