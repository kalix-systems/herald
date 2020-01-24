use super::*;

pub struct UserQObject;

pub struct UserEmitter {
    pub(super) qobject: Arc<AtomicPtr<UserQObject>>,
    pub(super) name_changed: fn(*mut UserQObject),
    pub(super) pairwise_conversation_id_changed: fn(*mut UserQObject),
    pub(super) profile_picture_changed: fn(*mut UserQObject),
    pub(super) user_color_changed: fn(*mut UserQObject),
    pub(super) user_id_changed: fn(*mut UserQObject),
}

impl UserEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> UserEmitter {
        UserEmitter {
            qobject: self.qobject.clone(),
            name_changed: self.name_changed,
            pairwise_conversation_id_changed: self.pairwise_conversation_id_changed,
            profile_picture_changed: self.profile_picture_changed,
            user_color_changed: self.user_color_changed,
            user_id_changed: self.user_id_changed,
        }
    }

    pub fn clear(&self) {
        let n: *const UserQObject = null();
        self.qobject.store(n as *mut UserQObject, Ordering::SeqCst);
    }

    pub fn name_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.name_changed)(ptr);
        }
    }

    pub fn pairwise_conversation_id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.pairwise_conversation_id_changed)(ptr);
        }
    }

    pub fn profile_picture_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.profile_picture_changed)(ptr);
        }
    }

    pub fn user_color_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.user_color_changed)(ptr);
        }
    }

    pub fn user_id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.user_id_changed)(ptr);
        }
    }
}

pub trait UserTrait {
    fn new(emit: UserEmitter) -> Self;

    fn emit(&mut self) -> &mut UserEmitter;

    fn name(&self) -> String;

    fn pairwise_conversation_id(&self) -> Vec<u8>;

    fn profile_picture(&self) -> Option<String>;

    fn user_color(&self) -> u32;

    fn set_user_color(
        &mut self,
        value: u32,
    );

    fn user_id(&self) -> Option<&str>;

    fn set_user_id(
        &mut self,
        value: Option<String>,
    );
}

#[no_mangle]
pub unsafe extern "C" fn user_new(ptr_bundle: *mut UserPtrBundle) -> *mut User {
    let d_user = user_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_user))
}

pub unsafe fn user_new_inner(ptr_bundle: *mut UserPtrBundle) -> User {
    let ptr_bundle = *ptr_bundle;

    let UserPtrBundle {
        user,
        user_name_changed,
        user_pairwise_conversation_id_changed,
        user_profile_picture_changed,
        user_user_color_changed,
        user_user_id_changed,
    } = ptr_bundle;
    let user_emit = UserEmitter {
        qobject: Arc::new(AtomicPtr::new(user)),
        name_changed: user_name_changed,
        pairwise_conversation_id_changed: user_pairwise_conversation_id_changed,
        profile_picture_changed: user_profile_picture_changed,
        user_color_changed: user_user_color_changed,
        user_id_changed: user_user_id_changed,
    };
    let d_user = User::new(user_emit);
    d_user
}

#[no_mangle]
pub unsafe extern "C" fn user_free(ptr: *mut User) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn user_name_get(
    ptr: *const User,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.name();
    let str_: *const c_char = value.as_ptr() as *const c_char;
    set(prop, str_, to_c_int(value.len()));
}

#[no_mangle]
pub unsafe extern "C" fn user_pairwise_conversation_id_get(
    ptr: *const User,
    prop: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.pairwise_conversation_id();
    let str_: *const c_char = value.as_ptr() as *const c_char;
    set(prop, str_, to_c_int(value.len()));
}

#[no_mangle]
pub unsafe extern "C" fn user_profile_picture_get(
    ptr: *const User,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.profile_picture();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn user_user_color_get(ptr: *const User) -> u32 {
    (&*ptr).user_color()
}

#[no_mangle]
pub unsafe extern "C" fn user_user_color_set(
    ptr: *mut User,
    value: u32,
) {
    (&mut *ptr).set_user_color(value)
}

#[no_mangle]
pub unsafe extern "C" fn user_user_id_get(
    ptr: *const User,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.user_id();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn user_user_id_set(
    ptr: *mut User,
    value: *const c_ushort,
    len: c_int,
) {
    let obj = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, value, len);
    obj.set_user_id(Some(s));
}

#[no_mangle]
pub unsafe extern "C" fn user_user_id_set_none(ptr: *mut User) {
    let obj = &mut *ptr;
    obj.set_user_id(None);
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct UserPtrBundle {
    user: *mut UserQObject,
    user_name_changed: fn(*mut UserQObject),
    user_pairwise_conversation_id_changed: fn(*mut UserQObject),
    user_profile_picture_changed: fn(*mut UserQObject),
    user_user_color_changed: fn(*mut UserQObject),
    user_user_id_changed: fn(*mut UserQObject),
}
