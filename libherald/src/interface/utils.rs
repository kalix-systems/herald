use super::*;

pub struct UtilsQObject;

pub struct UtilsEmitter {
    pub(super) qobject: Arc<AtomicPtr<UtilsQObject>>,
}

impl UtilsEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> UtilsEmitter {
        UtilsEmitter {
            qobject: self.qobject.clone(),
        }
    }

    pub fn clear(&self) {
        let n: *const UtilsQObject = null();
        self.qobject.store(n as *mut UtilsQObject, Ordering::SeqCst);
    }
}

pub trait UtilsTrait {
    fn new(emit: UtilsEmitter) -> Self;

    fn emit(&mut self) -> &mut UtilsEmitter;

    fn compare_byte_array(
        &self,
        bs1: &[u8],
        bs2: &[u8],
    ) -> bool;

    fn image_dimensions(
        &self,
        path: String,
    ) -> String;

    fn is_valid_rand_id(
        &self,
        bs: &[u8],
    ) -> bool;

    fn save_file(
        &self,
        fpath: String,
        target_path: String,
    ) -> bool;

    fn strip_url_prefix(
        &self,
        path: String,
    ) -> String;
}

#[no_mangle]
pub unsafe extern "C" fn utils_new(ptr_bundle: *mut UtilsPtrBundle) -> *mut Utils {
    let d_utils = utils_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_utils))
}

pub unsafe fn utils_new_inner(ptr_bundle: *mut UtilsPtrBundle) -> Utils {
    let ptr_bundle = *ptr_bundle;

    let UtilsPtrBundle { utils } = ptr_bundle;
    let utils_emit = UtilsEmitter {
        qobject: Arc::new(AtomicPtr::new(utils)),
    };
    let d_utils = Utils::new(utils_emit);
    d_utils
}

#[no_mangle]
pub unsafe extern "C" fn utils_free(ptr: *mut Utils) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn utils_compare_byte_array(
    ptr: *const Utils,
    bs1_str: *const c_char,
    bs1_len: c_int,
    bs2_str: *const c_char,
    bs2_len: c_int,
) -> bool {
    let obj = &*ptr;
    let bs1 = { qba_slice!(bs1_str, bs1_len) };
    let bs2 = { qba_slice!(bs2_str, bs2_len) };
    obj.compare_byte_array(bs1, bs2)
}

#[no_mangle]
pub unsafe extern "C" fn utils_image_dimensions(
    ptr: *const Utils,
    path_str: *const c_ushort,
    path_len: c_int,
    data: *mut QString,
    set: fn(*mut QString, str_: *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let mut path = String::new();
    set_string_from_utf16(&mut path, path_str, path_len);
    let ret = obj.image_dimensions(path);
    let str_: *const c_char = ret.as_ptr() as (*const c_char);
    set(data, str_, ret.len() as i32);
}

#[no_mangle]
pub unsafe extern "C" fn utils_is_valid_rand_id(
    ptr: *const Utils,
    bs_str: *const c_char,
    bs_len: c_int,
) -> bool {
    let obj = &*ptr;
    let bs = { qba_slice!(bs_str, bs_len) };
    obj.is_valid_rand_id(bs)
}

#[no_mangle]
pub unsafe extern "C" fn utils_save_file(
    ptr: *const Utils,
    fpath_str: *const c_ushort,
    fpath_len: c_int,
    target_path_str: *const c_ushort,
    target_path_len: c_int,
) -> bool {
    let obj = &*ptr;
    let mut fpath = String::new();
    set_string_from_utf16(&mut fpath, fpath_str, fpath_len);
    let mut target_path = String::new();
    set_string_from_utf16(&mut target_path, target_path_str, target_path_len);
    obj.save_file(fpath, target_path)
}

#[no_mangle]
pub unsafe extern "C" fn utils_strip_url_prefix(
    ptr: *const Utils,
    path_str: *const c_ushort,
    path_len: c_int,
    data: *mut QString,
    set: fn(*mut QString, str_: *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let mut path = String::new();
    set_string_from_utf16(&mut path, path_str, path_len);
    let ret = obj.strip_url_prefix(path);
    let str_: *const c_char = ret.as_ptr() as (*const c_char);
    set(data, str_, ret.len() as i32);
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct UtilsPtrBundle {
    utils: *mut UtilsQObject,
}
