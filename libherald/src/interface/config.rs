use super::*;

pub struct ConfigQObject;

pub struct ConfigEmitter {
    pub(super) qobject: Arc<AtomicPtr<ConfigQObject>>,
    pub(super) color_changed: fn(*mut ConfigQObject),
    pub(super) colorscheme_changed: fn(*mut ConfigQObject),
    pub(super) config_id_changed: fn(*mut ConfigQObject),
    pub(super) name_changed: fn(*mut ConfigQObject),
    pub(super) nts_conversation_id_changed: fn(*mut ConfigQObject),
    pub(super) profile_picture_changed: fn(*mut ConfigQObject),
}

impl ConfigEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> ConfigEmitter {
        ConfigEmitter {
            qobject: self.qobject.clone(),
            color_changed: self.color_changed,
            colorscheme_changed: self.colorscheme_changed,
            config_id_changed: self.config_id_changed,
            name_changed: self.name_changed,
            nts_conversation_id_changed: self.nts_conversation_id_changed,
            profile_picture_changed: self.profile_picture_changed,
        }
    }

    pub fn clear(&self) {
        let n: *const ConfigQObject = null();
        self.qobject
            .store(n as *mut ConfigQObject, Ordering::SeqCst);
    }

    pub fn color_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.color_changed)(ptr);
        }
    }

    pub fn colorscheme_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.colorscheme_changed)(ptr);
        }
    }

    pub fn config_id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.config_id_changed)(ptr);
        }
    }

    pub fn name_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.name_changed)(ptr);
        }
    }

    pub fn nts_conversation_id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.nts_conversation_id_changed)(ptr);
        }
    }

    pub fn profile_picture_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.profile_picture_changed)(ptr);
        }
    }
}

pub trait ConfigTrait {
    fn new(emit: ConfigEmitter) -> Self;

    fn emit(&mut self) -> &mut ConfigEmitter;

    fn color(&self) -> u32;

    fn set_color(
        &mut self,
        value: u32,
    );

    fn colorscheme(&self) -> u32;

    fn set_colorscheme(
        &mut self,
        value: u32,
    );

    fn config_id(&self) -> &str;

    fn name(&self) -> &str;

    fn set_name(
        &mut self,
        value: String,
    );

    fn nts_conversation_id(&self) -> &[u8];

    fn profile_picture(&self) -> Option<&str>;

    fn set_profile_picture(
        &mut self,
        value: Option<String>,
    );
}

#[no_mangle]
pub unsafe extern "C" fn config_new(ptr_bundle: *mut ConfigPtrBundle) -> *mut Config {
    let d_config = config_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_config))
}

pub unsafe fn config_new_inner(ptr_bundle: *mut ConfigPtrBundle) -> Config {
    let ptr_bundle = *ptr_bundle;

    let ConfigPtrBundle {
        config,
        config_color_changed,
        config_colorscheme_changed,
        config_config_id_changed,
        config_name_changed,
        config_nts_conversation_id_changed,
        config_profile_picture_changed,
    } = ptr_bundle;
    let config_emit = ConfigEmitter {
        qobject: Arc::new(AtomicPtr::new(config)),
        color_changed: config_color_changed,
        colorscheme_changed: config_colorscheme_changed,
        config_id_changed: config_config_id_changed,
        name_changed: config_name_changed,
        nts_conversation_id_changed: config_nts_conversation_id_changed,
        profile_picture_changed: config_profile_picture_changed,
    };
    let d_config = Config::new(config_emit);
    d_config
}

#[no_mangle]
pub unsafe extern "C" fn config_free(ptr: *mut Config) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn config_color_get(ptr: *const Config) -> u32 {
    (&*ptr).color()
}

#[no_mangle]
pub unsafe extern "C" fn config_color_set(
    ptr: *mut Config,
    value: u32,
) {
    (&mut *ptr).set_color(value)
}

#[no_mangle]
pub unsafe extern "C" fn config_colorscheme_get(ptr: *const Config) -> u32 {
    (&*ptr).colorscheme()
}

#[no_mangle]
pub unsafe extern "C" fn config_colorscheme_set(
    ptr: *mut Config,
    value: u32,
) {
    (&mut *ptr).set_colorscheme(value)
}

#[no_mangle]
pub unsafe extern "C" fn config_config_id_get(
    ptr: *const Config,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.config_id();
    let str_: *const c_char = value.as_ptr() as *const c_char;
    set(prop, str_, to_c_int(value.len()));
}

#[no_mangle]
pub unsafe extern "C" fn config_name_get(
    ptr: *const Config,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.name();
    let str_: *const c_char = value.as_ptr() as *const c_char;
    set(prop, str_, to_c_int(value.len()));
}

#[no_mangle]
pub unsafe extern "C" fn config_name_set(
    ptr: *mut Config,
    value: *const c_ushort,
    len: c_int,
) {
    let obj = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, value, len);
    obj.set_name(s);
}

#[no_mangle]
pub unsafe extern "C" fn config_nts_conversation_id_get(
    ptr: *const Config,
    prop: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.nts_conversation_id();
    let str_: *const c_char = value.as_ptr() as *const c_char;
    set(prop, str_, to_c_int(value.len()));
}

#[no_mangle]
pub unsafe extern "C" fn config_profile_picture_get(
    ptr: *const Config,
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
pub unsafe extern "C" fn config_profile_picture_set(
    ptr: *mut Config,
    value: *const c_ushort,
    len: c_int,
) {
    let obj = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, value, len);
    obj.set_profile_picture(Some(s));
}

#[no_mangle]
pub unsafe extern "C" fn config_profile_picture_set_none(ptr: *mut Config) {
    let obj = &mut *ptr;
    obj.set_profile_picture(None);
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ConfigPtrBundle {
    config: *mut ConfigQObject,
    config_color_changed: fn(*mut ConfigQObject),
    config_colorscheme_changed: fn(*mut ConfigQObject),
    config_config_id_changed: fn(*mut ConfigQObject),
    config_name_changed: fn(*mut ConfigQObject),
    config_nts_conversation_id_changed: fn(*mut ConfigQObject),
    config_profile_picture_changed: fn(*mut ConfigQObject),
}
