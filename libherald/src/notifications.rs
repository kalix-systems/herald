use crate::interface::{NotificationsEmitter as Emitter, NotificationsTrait as Interface};
use std::collections::VecDeque;

/// A Notifications queue used only on windows
pub struct Notifications {
    emit: Emitter,
    notifications: VecDeque<json::JsonValue>,
}

impl Interface for Notifications {
    fn new(emit: Emitter) -> Self {
        Notifications {
            emit,
            notifications: VecDeque::new(),
        }
    }

    fn next_notif(&mut self) -> String {
        self.notifications
            .pop_back()
            .map(|v| v.dump())
            .unwrap_or_default()
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }
}

impl Notifications {
    pub(crate) fn handle_notifications(
        &mut self,
        notif: json::JsonValue,
    ) {
        self.notifications.push_front(notif);
        self.emit.notify();
    }
}
