use super::*;

pub struct UsersQObject;

pub struct UsersEmitter {
    pub(super) qobject: Arc<AtomicPtr<UsersQObject>>,
    pub(super) filter_changed: fn(*mut UsersQObject),
    pub(super) filter_regex_changed: fn(*mut UsersQObject),
    pub(super) new_data_ready: fn(*mut UsersQObject),
}

impl UsersEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> UsersEmitter {
        UsersEmitter {
            qobject: self.qobject.clone(),
            filter_changed: self.filter_changed,
            filter_regex_changed: self.filter_regex_changed,
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const UsersQObject = null();
        self.qobject.store(n as *mut UsersQObject, Ordering::SeqCst);
    }

    pub fn filter_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.filter_changed)(ptr);
        }
    }

    pub fn filter_regex_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.filter_regex_changed)(ptr);
        }
    }

    pub fn new_data_ready(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.new_data_ready)(ptr);
        }
    }
}

#[derive(Clone)]
pub struct UsersList {
    pub(super) qobject: *mut UsersQObject,
    pub(super) layout_about_to_be_changed: fn(*mut UsersQObject),
    pub(super) layout_changed: fn(*mut UsersQObject),
    pub(super) begin_reset_model: fn(*mut UsersQObject),
    pub(super) end_reset_model: fn(*mut UsersQObject),
    pub(super) end_insert_rows: fn(*mut UsersQObject),
    pub(super) end_move_rows: fn(*mut UsersQObject),
    pub(super) end_remove_rows: fn(*mut UsersQObject),
    pub(super) begin_insert_rows: fn(*mut UsersQObject, usize, usize),
    pub(super) begin_remove_rows: fn(*mut UsersQObject, usize, usize),
    pub(super) data_changed: fn(*mut UsersQObject, usize, usize),
    pub(super) begin_move_rows: fn(*mut UsersQObject, usize, usize, usize),
}

impl UsersList {
    pub fn layout_about_to_be_changed(&mut self) {
        if !self.qobject.is_null() {
            (self.layout_about_to_be_changed)(self.qobject);
        }
    }

    pub fn layout_changed(&mut self) {
        if !self.qobject.is_null() {
            (self.layout_changed)(self.qobject)
        }
    }

    pub fn begin_reset_model(&mut self) {
        if !self.qobject.is_null() {
            (self.begin_reset_model)(self.qobject);
        }
    }

    pub fn end_reset_model(&mut self) {
        if !self.qobject.is_null() {
            (self.end_reset_model)(self.qobject);
        }
    }

    pub fn end_insert_rows(&mut self) {
        if !self.qobject.is_null() {
            (self.end_insert_rows)(self.qobject);
        }
    }

    pub fn end_move_rows(&mut self) {
        if !self.qobject.is_null() {
            (self.end_move_rows)(self.qobject);
        }
    }

    pub fn end_remove_rows(&mut self) {
        if !self.qobject.is_null() {
            (self.end_remove_rows)(self.qobject);
        }
    }

    pub fn begin_insert_rows(
        &mut self,
        first: usize,
        last: usize,
    ) {
        if !self.qobject.is_null() {
            (self.begin_insert_rows)(self.qobject, first, last);
        }
    }

    pub fn begin_remove_rows(
        &mut self,
        first: usize,
        last: usize,
    ) {
        if !self.qobject.is_null() {
            (self.begin_remove_rows)(self.qobject, first, last);
        }
    }

    pub fn data_changed(
        &mut self,
        first: usize,
        last: usize,
    ) {
        if !self.qobject.is_null() {
            (self.data_changed)(self.qobject, first, last);
        }
    }

    pub fn begin_move_rows(
        &mut self,
        first: usize,
        last: usize,
        destination: usize,
    ) {
        if !self.qobject.is_null() {
            (self.begin_move_rows)(self.qobject, first, last, destination);
        }
    }
}

pub trait UsersTrait {
    fn new(
        emit: UsersEmitter,
        model: UsersList,
    ) -> Self;

    fn emit(&mut self) -> &mut UsersEmitter;

    fn filter(&self) -> &str;

    fn set_filter(
        &mut self,
        value: String,
    );

    fn filter_regex(&self) -> bool;

    fn set_filter_regex(
        &mut self,
        value: bool,
    );

    fn add(
        &mut self,
        id: String,
    ) -> Vec<u8>;

    fn clear_filter(&mut self) -> ();

    fn color_by_id(
        &self,
        id: String,
    ) -> u32;

    fn index_by_id(
        &self,
        id: String,
    ) -> i64;

    fn name_by_id(
        &self,
        id: String,
    ) -> String;

    fn profile_picture_by_id(
        &self,
        id: String,
    ) -> String;

    fn toggle_filter_regex(&mut self) -> bool;

    fn row_count(&self) -> usize;

    fn insert_rows(
        &mut self,
        _row: usize,
        _count: usize,
    ) -> bool {
        false
    }

    fn remove_rows(
        &mut self,
        _row: usize,
        _count: usize,
    ) -> bool {
        false
    }

    fn can_fetch_more(&self) -> bool {
        false
    }

    fn fetch_more(&mut self) {}

    fn sort(
        &mut self,
        _: u8,
        _: SortOrder,
    ) {
    }

    fn color(
        &self,
        index: usize,
    ) -> u32;

    fn set_color(
        &mut self,
        index: usize,
        _: u32,
    ) -> bool;

    fn matched(
        &self,
        index: usize,
    ) -> bool;

    fn name(
        &self,
        index: usize,
    ) -> String;

    fn pairwise_conversation_id(
        &self,
        index: usize,
    ) -> Vec<u8>;

    fn profile_picture(
        &self,
        index: usize,
    ) -> Option<String>;

    fn status(
        &self,
        index: usize,
    ) -> u8;

    fn set_status(
        &mut self,
        index: usize,
        _: u8,
    ) -> bool;

    fn user_id(
        &self,
        index: usize,
    ) -> &str;
}

#[no_mangle]
pub unsafe extern "C" fn users_new(ptr_bundle: *mut UsersPtrBundle) -> *mut Users {
    let d_users = users_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_users))
}

pub unsafe fn users_new_inner(ptr_bundle: *mut UsersPtrBundle) -> Users {
    let ptr_bundle = *ptr_bundle;

    let UsersPtrBundle {
        users,
        users_filter_changed,
        users_filter_regex_changed,
        users_new_data_ready,
        users_layout_about_to_be_changed,
        users_layout_changed,
        users_data_changed,
        users_begin_reset_model,
        users_end_reset_model,
        users_begin_insert_rows,
        users_end_insert_rows,
        users_begin_move_rows,
        users_end_move_rows,
        users_begin_remove_rows,
        users_end_remove_rows,
    } = ptr_bundle;
    let users_emit = UsersEmitter {
        qobject: Arc::new(AtomicPtr::new(users)),
        filter_changed: users_filter_changed,
        filter_regex_changed: users_filter_regex_changed,
        new_data_ready: users_new_data_ready,
    };
    let model = UsersList {
        qobject: users,
        layout_about_to_be_changed: users_layout_about_to_be_changed,
        layout_changed: users_layout_changed,
        data_changed: users_data_changed,
        begin_reset_model: users_begin_reset_model,
        end_reset_model: users_end_reset_model,
        begin_insert_rows: users_begin_insert_rows,
        end_insert_rows: users_end_insert_rows,
        begin_move_rows: users_begin_move_rows,
        end_move_rows: users_end_move_rows,
        begin_remove_rows: users_begin_remove_rows,
        end_remove_rows: users_end_remove_rows,
    };
    let d_users = Users::new(users_emit, model);
    d_users
}

#[no_mangle]
pub unsafe extern "C" fn users_free(ptr: *mut Users) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn users_add(
    ptr: *mut Users,
    id_str: *const c_ushort,
    id_len: c_int,
    data: *mut QByteArray,
    set: fn(*mut QByteArray, str_: *const c_char, len: c_int),
) {
    let obj = &mut *ptr;
    let mut id = String::new();
    set_string_from_utf16(&mut id, id_str, id_len);
    let ret = obj.add(id);
    let str_: *const c_char = ret.as_ptr() as (*const c_char);
    set(data, str_, ret.len() as i32);
}

#[no_mangle]
pub unsafe extern "C" fn users_clear_filter(ptr: *mut Users) {
    let obj = &mut *ptr;
    obj.clear_filter()
}

#[no_mangle]
pub unsafe extern "C" fn users_color_by_id(
    ptr: *const Users,
    id_str: *const c_ushort,
    id_len: c_int,
) -> u32 {
    let obj = &*ptr;
    let mut id = String::new();
    set_string_from_utf16(&mut id, id_str, id_len);
    obj.color_by_id(id)
}

#[no_mangle]
pub unsafe extern "C" fn users_index_by_id(
    ptr: *const Users,
    id_str: *const c_ushort,
    id_len: c_int,
) -> i64 {
    let obj = &*ptr;
    let mut id = String::new();
    set_string_from_utf16(&mut id, id_str, id_len);
    obj.index_by_id(id)
}

#[no_mangle]
pub unsafe extern "C" fn users_name_by_id(
    ptr: *const Users,
    id_str: *const c_ushort,
    id_len: c_int,
    data: *mut QString,
    set: fn(*mut QString, str_: *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let mut id = String::new();
    set_string_from_utf16(&mut id, id_str, id_len);
    let ret = obj.name_by_id(id);
    let str_: *const c_char = ret.as_ptr() as (*const c_char);
    set(data, str_, ret.len() as i32);
}

#[no_mangle]
pub unsafe extern "C" fn users_profile_picture_by_id(
    ptr: *const Users,
    id_str: *const c_ushort,
    id_len: c_int,
    data: *mut QString,
    set: fn(*mut QString, str_: *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let mut id = String::new();
    set_string_from_utf16(&mut id, id_str, id_len);
    let ret = obj.profile_picture_by_id(id);
    let str_: *const c_char = ret.as_ptr() as (*const c_char);
    set(data, str_, ret.len() as i32);
}

#[no_mangle]
pub unsafe extern "C" fn users_toggle_filter_regex(ptr: *mut Users) -> bool {
    let obj = &mut *ptr;
    obj.toggle_filter_regex()
}

#[no_mangle]
pub unsafe extern "C" fn users_filter_get(
    ptr: *const Users,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.filter();
    let str_: *const c_char = value.as_ptr() as *const c_char;
    set(prop, str_, to_c_int(value.len()));
}

#[no_mangle]
pub unsafe extern "C" fn users_filter_set(
    ptr: *mut Users,
    value: *const c_ushort,
    len: c_int,
) {
    let obj = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, value, len);
    obj.set_filter(s);
}

#[no_mangle]
pub unsafe extern "C" fn users_filter_regex_get(ptr: *const Users) -> bool {
    (&*ptr).filter_regex()
}

#[no_mangle]
pub unsafe extern "C" fn users_filter_regex_set(
    ptr: *mut Users,
    value: bool,
) {
    (&mut *ptr).set_filter_regex(value)
}

#[no_mangle]
pub unsafe extern "C" fn users_row_count(ptr: *const Users) -> c_int {
    to_c_int((&*ptr).row_count())
}

#[no_mangle]
pub unsafe extern "C" fn users_insert_rows(
    ptr: *mut Users,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).insert_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn users_remove_rows(
    ptr: *mut Users,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).remove_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn users_can_fetch_more(ptr: *const Users) -> bool {
    (&*ptr).can_fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn users_fetch_more(ptr: *mut Users) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn users_sort(
    ptr: *mut Users,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn users_data_color(
    ptr: *const Users,
    row: c_int,
) -> u32 {
    let obj = &*ptr;
    obj.color(to_usize(row).unwrap_or(0))
}

#[no_mangle]
pub unsafe extern "C" fn users_set_data_color(
    ptr: *mut Users,
    row: c_int,
    value: u32,
) -> bool {
    (&mut *ptr).set_color(to_usize(row).unwrap_or(0), value)
}

#[no_mangle]
pub unsafe extern "C" fn users_data_matched(
    ptr: *const Users,
    row: c_int,
) -> bool {
    let obj = &*ptr;
    obj.matched(to_usize(row).unwrap_or(0))
}

#[no_mangle]
pub unsafe extern "C" fn users_data_name(
    ptr: *const Users,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.name(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn users_data_pairwise_conversation_id(
    ptr: *const Users,
    row: c_int,
    d: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.pairwise_conversation_id(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn users_data_profile_picture(
    ptr: *const Users,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.profile_picture(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn users_data_status(
    ptr: *const Users,
    row: c_int,
) -> u8 {
    let obj = &*ptr;
    obj.status(to_usize(row).unwrap_or(0))
}

#[no_mangle]
pub unsafe extern "C" fn users_set_data_status(
    ptr: *mut Users,
    row: c_int,
    value: u8,
) -> bool {
    (&mut *ptr).set_status(to_usize(row).unwrap_or(0), value)
}

#[no_mangle]
pub unsafe extern "C" fn users_data_user_id(
    ptr: *const Users,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.user_id(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct UsersPtrBundle {
    users: *mut UsersQObject,
    users_filter_changed: fn(*mut UsersQObject),
    users_filter_regex_changed: fn(*mut UsersQObject),
    users_new_data_ready: fn(*mut UsersQObject),
    users_layout_about_to_be_changed: fn(*mut UsersQObject),
    users_layout_changed: fn(*mut UsersQObject),
    users_data_changed: fn(*mut UsersQObject, usize, usize),
    users_begin_reset_model: fn(*mut UsersQObject),
    users_end_reset_model: fn(*mut UsersQObject),
    users_begin_insert_rows: fn(*mut UsersQObject, usize, usize),
    users_end_insert_rows: fn(*mut UsersQObject),
    users_begin_move_rows: fn(*mut UsersQObject, usize, usize, usize),
    users_end_move_rows: fn(*mut UsersQObject),
    users_begin_remove_rows: fn(*mut UsersQObject, usize, usize),
    users_end_remove_rows: fn(*mut UsersQObject),
}
