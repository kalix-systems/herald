use super::*;

pub struct NotificationsQObject;

pub struct NotificationsEmitter {
    pub(super) qobject: Arc<AtomicPtr<NotificationsQObject>>,
    pub(super) notify: fn(*mut NotificationsQObject),
}

impl NotificationsEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> NotificationsEmitter {
        NotificationsEmitter {
            qobject: self.qobject.clone(),
            notify: self.notify,
        }
    }

    pub fn clear(&self) {
        let n: *const NotificationsQObject = null();
        self.qobject
            .store(n as *mut NotificationsQObject, Ordering::SeqCst);
    }

    pub fn notify(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.notify)(ptr);
        }
    }
}

pub trait NotificationsTrait {
    fn new(emit: NotificationsEmitter) -> Self;

    fn emit(&mut self) -> &mut NotificationsEmitter;

    fn next_notif(&mut self) -> String;
}

#[no_mangle]
pub unsafe extern "C" fn notifications_new(
    ptr_bundle: *mut NotificationsPtrBundle
) -> *mut Notifications {
    let d_notifications = notifications_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_notifications))
}

pub unsafe fn notifications_new_inner(ptr_bundle: *mut NotificationsPtrBundle) -> Notifications {
    let ptr_bundle = *ptr_bundle;

    let NotificationsPtrBundle {
        notifications,
        notifications_notify,
    } = ptr_bundle;
    let notifications_emit = NotificationsEmitter {
        qobject: Arc::new(AtomicPtr::new(notifications)),
        notify: notifications_notify,
    };
    let d_notifications = Notifications::new(notifications_emit);
    d_notifications
}

#[no_mangle]
pub unsafe extern "C" fn notifications_free(ptr: *mut Notifications) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn notifications_next_notif(
    ptr: *mut Notifications,
    data: *mut QString,
    set: fn(*mut QString, str_: *const c_char, len: c_int),
) {
    let obj = &mut *ptr;
    let ret = obj.next_notif();
    let str_: *const c_char = ret.as_ptr() as (*const c_char);
    set(data, str_, ret.len() as i32);
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct NotificationsPtrBundle {
    notifications: *mut NotificationsQObject,
    notifications_notify: fn(*mut NotificationsQObject),
}
