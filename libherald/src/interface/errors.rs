use super::*;

pub struct ErrorsQObject;

pub struct ErrorsEmitter {
    pub(super) qobject: Arc<AtomicPtr<ErrorsQObject>>,
    pub(super) try_poll_changed: fn(*mut ErrorsQObject),
}

impl ErrorsEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> ErrorsEmitter {
        ErrorsEmitter {
            qobject: self.qobject.clone(),
            try_poll_changed: self.try_poll_changed,
        }
    }

    pub fn clear(&self) {
        let n: *const ErrorsQObject = null();
        self.qobject
            .store(n as *mut ErrorsQObject, Ordering::SeqCst);
    }

    pub fn try_poll_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.try_poll_changed)(ptr);
        }
    }
}

pub trait ErrorsTrait {
    fn new(emit: ErrorsEmitter) -> Self;

    fn emit(&mut self) -> &mut ErrorsEmitter;

    fn try_poll(&self) -> u8;

    fn next_error(&mut self) -> String;
}

#[no_mangle]
pub unsafe extern "C" fn errors_new(ptr_bundle: *mut ErrorsPtrBundle) -> *mut Errors {
    let d_errors = errors_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_errors))
}

pub unsafe fn errors_new_inner(ptr_bundle: *mut ErrorsPtrBundle) -> Errors {
    let ptr_bundle = *ptr_bundle;

    let ErrorsPtrBundle {
        errors,
        errors_try_poll_changed,
    } = ptr_bundle;
    let errors_emit = ErrorsEmitter {
        qobject: Arc::new(AtomicPtr::new(errors)),
        try_poll_changed: errors_try_poll_changed,
    };
    let d_errors = Errors::new(errors_emit);
    d_errors
}

#[no_mangle]
pub unsafe extern "C" fn errors_free(ptr: *mut Errors) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn errors_next_error(
    ptr: *mut Errors,
    data: *mut QString,
    set: fn(*mut QString, str_: *const c_char, len: c_int),
) {
    let obj = &mut *ptr;
    let ret = obj.next_error();
    let str_: *const c_char = ret.as_ptr() as (*const c_char);
    set(data, str_, ret.len() as i32);
}

#[no_mangle]
pub unsafe extern "C" fn errors_try_poll_get(ptr: *const Errors) -> u8 {
    (&*ptr).try_poll()
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ErrorsPtrBundle {
    errors: *mut ErrorsQObject,
    errors_try_poll_changed: fn(*mut ErrorsQObject),
}
