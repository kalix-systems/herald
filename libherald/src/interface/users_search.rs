use super::*;

pub struct UsersSearchQObject;

pub struct UsersSearchEmitter {
    pub(super) qobject: Arc<AtomicPtr<UsersSearchQObject>>,
    pub(super) filter_changed: fn(*mut UsersSearchQObject),
    pub(super) new_data_ready: fn(*mut UsersSearchQObject),
}

impl UsersSearchEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> UsersSearchEmitter {
        UsersSearchEmitter {
            qobject: self.qobject.clone(),
            filter_changed: self.filter_changed,
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const UsersSearchQObject = null();
        self.qobject
            .store(n as *mut UsersSearchQObject, Ordering::SeqCst);
    }

    pub fn filter_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.filter_changed)(ptr);
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
pub struct UsersSearchList {
    pub(super) qobject: *mut UsersSearchQObject,
    pub(super) layout_about_to_be_changed: fn(*mut UsersSearchQObject),
    pub(super) layout_changed: fn(*mut UsersSearchQObject),
    pub(super) begin_reset_model: fn(*mut UsersSearchQObject),
    pub(super) end_reset_model: fn(*mut UsersSearchQObject),
    pub(super) end_insert_rows: fn(*mut UsersSearchQObject),
    pub(super) end_move_rows: fn(*mut UsersSearchQObject),
    pub(super) end_remove_rows: fn(*mut UsersSearchQObject),
    pub(super) begin_insert_rows: fn(*mut UsersSearchQObject, usize, usize),
    pub(super) begin_remove_rows: fn(*mut UsersSearchQObject, usize, usize),
    pub(super) data_changed: fn(*mut UsersSearchQObject, usize, usize),
    pub(super) begin_move_rows: fn(*mut UsersSearchQObject, usize, usize, usize),
}

impl UsersSearchList {
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

pub trait UsersSearchTrait {
    fn new(
        emit: UsersSearchEmitter,
        model: UsersSearchList,
    ) -> Self;

    fn emit(&mut self) -> &mut UsersSearchEmitter;

    fn filter(&self) -> Option<&str>;

    fn set_filter(
        &mut self,
        value: Option<String>,
    );

    fn clear_filter(&mut self) -> ();

    fn refresh(&mut self) -> ();

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

    fn matched(
        &self,
        index: usize,
    ) -> bool;

    fn selected(
        &self,
        index: usize,
    ) -> bool;

    fn set_selected(
        &mut self,
        index: usize,
        _: bool,
    ) -> bool;

    fn user_id(
        &self,
        index: usize,
    ) -> Option<&str>;
}

#[no_mangle]
pub unsafe extern "C" fn users_search_new(
    ptr_bundle: *mut UsersSearchPtrBundle
) -> *mut UsersSearch {
    let d_users_search = users_search_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_users_search))
}

pub unsafe fn users_search_new_inner(ptr_bundle: *mut UsersSearchPtrBundle) -> UsersSearch {
    let ptr_bundle = *ptr_bundle;

    let UsersSearchPtrBundle {
        users_search,
        users_search_filter_changed,
        users_search_new_data_ready,
        users_search_layout_about_to_be_changed,
        users_search_layout_changed,
        users_search_data_changed,
        users_search_begin_reset_model,
        users_search_end_reset_model,
        users_search_begin_insert_rows,
        users_search_end_insert_rows,
        users_search_begin_move_rows,
        users_search_end_move_rows,
        users_search_begin_remove_rows,
        users_search_end_remove_rows,
    } = ptr_bundle;
    let users_search_emit = UsersSearchEmitter {
        qobject: Arc::new(AtomicPtr::new(users_search)),
        filter_changed: users_search_filter_changed,
        new_data_ready: users_search_new_data_ready,
    };
    let model = UsersSearchList {
        qobject: users_search,
        layout_about_to_be_changed: users_search_layout_about_to_be_changed,
        layout_changed: users_search_layout_changed,
        data_changed: users_search_data_changed,
        begin_reset_model: users_search_begin_reset_model,
        end_reset_model: users_search_end_reset_model,
        begin_insert_rows: users_search_begin_insert_rows,
        end_insert_rows: users_search_end_insert_rows,
        begin_move_rows: users_search_begin_move_rows,
        end_move_rows: users_search_end_move_rows,
        begin_remove_rows: users_search_begin_remove_rows,
        end_remove_rows: users_search_end_remove_rows,
    };
    let d_users_search = UsersSearch::new(users_search_emit, model);
    d_users_search
}

#[no_mangle]
pub unsafe extern "C" fn users_search_free(ptr: *mut UsersSearch) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn users_search_clear_filter(ptr: *mut UsersSearch) {
    let obj = &mut *ptr;
    obj.clear_filter()
}

#[no_mangle]
pub unsafe extern "C" fn users_search_refresh(ptr: *mut UsersSearch) {
    let obj = &mut *ptr;
    obj.refresh()
}

#[no_mangle]
pub unsafe extern "C" fn users_search_filter_get(
    ptr: *const UsersSearch,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.filter();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn users_search_filter_set(
    ptr: *mut UsersSearch,
    value: *const c_ushort,
    len: c_int,
) {
    let obj = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, value, len);
    obj.set_filter(Some(s));
}

#[no_mangle]
pub unsafe extern "C" fn users_search_filter_set_none(ptr: *mut UsersSearch) {
    let obj = &mut *ptr;
    obj.set_filter(None);
}

#[no_mangle]
pub unsafe extern "C" fn users_search_row_count(ptr: *const UsersSearch) -> c_int {
    to_c_int((&*ptr).row_count())
}

#[no_mangle]
pub unsafe extern "C" fn users_search_insert_rows(
    ptr: *mut UsersSearch,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).insert_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn users_search_remove_rows(
    ptr: *mut UsersSearch,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).remove_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn users_search_can_fetch_more(ptr: *const UsersSearch) -> bool {
    (&*ptr).can_fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn users_search_fetch_more(ptr: *mut UsersSearch) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn users_search_sort(
    ptr: *mut UsersSearch,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn users_search_data_matched(
    ptr: *const UsersSearch,
    row: c_int,
) -> bool {
    let obj = &*ptr;
    obj.matched(to_usize(row).unwrap_or(0))
}

#[no_mangle]
pub unsafe extern "C" fn users_search_data_selected(
    ptr: *const UsersSearch,
    row: c_int,
) -> bool {
    let obj = &*ptr;
    obj.selected(to_usize(row).unwrap_or(0))
}

#[no_mangle]
pub unsafe extern "C" fn users_search_set_data_selected(
    ptr: *mut UsersSearch,
    row: c_int,
    value: bool,
) -> bool {
    (&mut *ptr).set_selected(to_usize(row).unwrap_or(0), value)
}

#[no_mangle]
pub unsafe extern "C" fn users_search_data_user_id(
    ptr: *const UsersSearch,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.user_id(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct UsersSearchPtrBundle {
    users_search: *mut UsersSearchQObject,
    users_search_filter_changed: fn(*mut UsersSearchQObject),
    users_search_new_data_ready: fn(*mut UsersSearchQObject),
    users_search_layout_about_to_be_changed: fn(*mut UsersSearchQObject),
    users_search_layout_changed: fn(*mut UsersSearchQObject),
    users_search_data_changed: fn(*mut UsersSearchQObject, usize, usize),
    users_search_begin_reset_model: fn(*mut UsersSearchQObject),
    users_search_end_reset_model: fn(*mut UsersSearchQObject),
    users_search_begin_insert_rows: fn(*mut UsersSearchQObject, usize, usize),
    users_search_end_insert_rows: fn(*mut UsersSearchQObject),
    users_search_begin_move_rows: fn(*mut UsersSearchQObject, usize, usize, usize),
    users_search_end_move_rows: fn(*mut UsersSearchQObject),
    users_search_begin_remove_rows: fn(*mut UsersSearchQObject, usize, usize),
    users_search_end_remove_rows: fn(*mut UsersSearchQObject),
}
