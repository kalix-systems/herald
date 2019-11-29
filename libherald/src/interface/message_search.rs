use super::*;

pub struct MessageSearchQObject;

pub struct MessageSearchEmitter {
    pub(super) qobject: Arc<AtomicPtr<MessageSearchQObject>>,
    pub(super) regex_search_changed: fn(*mut MessageSearchQObject),
    pub(super) search_pattern_changed: fn(*mut MessageSearchQObject),
    pub(super) new_data_ready: fn(*mut MessageSearchQObject),
}

impl MessageSearchEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> MessageSearchEmitter {
        MessageSearchEmitter {
            qobject: self.qobject.clone(),
            regex_search_changed: self.regex_search_changed,
            search_pattern_changed: self.search_pattern_changed,
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const MessageSearchQObject = null();
        self.qobject
            .store(n as *mut MessageSearchQObject, Ordering::SeqCst);
    }

    pub fn regex_search_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.regex_search_changed)(ptr);
        }
    }

    pub fn search_pattern_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.search_pattern_changed)(ptr);
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
pub struct MessageSearchList {
    pub(super) qobject: *mut MessageSearchQObject,
    pub(super) layout_about_to_be_changed: fn(*mut MessageSearchQObject),
    pub(super) layout_changed: fn(*mut MessageSearchQObject),
    pub(super) begin_reset_model: fn(*mut MessageSearchQObject),
    pub(super) end_reset_model: fn(*mut MessageSearchQObject),
    pub(super) end_insert_rows: fn(*mut MessageSearchQObject),
    pub(super) end_move_rows: fn(*mut MessageSearchQObject),
    pub(super) end_remove_rows: fn(*mut MessageSearchQObject),
    pub(super) begin_insert_rows: fn(*mut MessageSearchQObject, usize, usize),
    pub(super) begin_remove_rows: fn(*mut MessageSearchQObject, usize, usize),
    pub(super) data_changed: fn(*mut MessageSearchQObject, usize, usize),
    pub(super) begin_move_rows: fn(*mut MessageSearchQObject, usize, usize, usize),
}

impl MessageSearchList {
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

pub trait MessageSearchTrait {
    fn new(
        emit: MessageSearchEmitter,
        model: MessageSearchList,
    ) -> Self;

    fn emit(&mut self) -> &mut MessageSearchEmitter;

    fn regex_search(&self) -> Option<bool>;

    fn set_regex_search(
        &mut self,
        value: Option<bool>,
    );

    fn search_pattern(&self) -> Option<&str>;

    fn set_search_pattern(
        &mut self,
        value: Option<String>,
    );

    fn clear_search(&mut self) -> ();

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

    fn author(
        &self,
        index: usize,
    ) -> Option<&str>;

    fn body(
        &self,
        index: usize,
    ) -> Option<&str>;

    fn conversation(
        &self,
        index: usize,
    ) -> Option<&[u8]>;

    fn conversation_color(
        &self,
        index: usize,
    ) -> Option<u32>;

    fn conversation_pairwise(
        &self,
        index: usize,
    ) -> Option<bool>;

    fn conversation_picture(
        &self,
        index: usize,
    ) -> Option<String>;

    fn conversation_title(
        &self,
        index: usize,
    ) -> Option<String>;

    fn has_attachments(
        &self,
        index: usize,
    ) -> Option<bool>;

    fn msg_id(
        &self,
        index: usize,
    ) -> Option<&[u8]>;

    fn time(
        &self,
        index: usize,
    ) -> Option<i64>;
}

#[no_mangle]
pub unsafe extern "C" fn message_search_new(
    ptr_bundle: *mut MessageSearchPtrBundle
) -> *mut MessageSearch {
    let d_message_search = message_search_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_message_search))
}

pub unsafe fn message_search_new_inner(ptr_bundle: *mut MessageSearchPtrBundle) -> MessageSearch {
    let ptr_bundle = *ptr_bundle;

    let MessageSearchPtrBundle {
        message_search,
        message_search_regex_search_changed,
        message_search_search_pattern_changed,
        message_search_new_data_ready,
        message_search_layout_about_to_be_changed,
        message_search_layout_changed,
        message_search_data_changed,
        message_search_begin_reset_model,
        message_search_end_reset_model,
        message_search_begin_insert_rows,
        message_search_end_insert_rows,
        message_search_begin_move_rows,
        message_search_end_move_rows,
        message_search_begin_remove_rows,
        message_search_end_remove_rows,
    } = ptr_bundle;
    let message_search_emit = MessageSearchEmitter {
        qobject: Arc::new(AtomicPtr::new(message_search)),
        regex_search_changed: message_search_regex_search_changed,
        search_pattern_changed: message_search_search_pattern_changed,
        new_data_ready: message_search_new_data_ready,
    };
    let model = MessageSearchList {
        qobject: message_search,
        layout_about_to_be_changed: message_search_layout_about_to_be_changed,
        layout_changed: message_search_layout_changed,
        data_changed: message_search_data_changed,
        begin_reset_model: message_search_begin_reset_model,
        end_reset_model: message_search_end_reset_model,
        begin_insert_rows: message_search_begin_insert_rows,
        end_insert_rows: message_search_end_insert_rows,
        begin_move_rows: message_search_begin_move_rows,
        end_move_rows: message_search_end_move_rows,
        begin_remove_rows: message_search_begin_remove_rows,
        end_remove_rows: message_search_end_remove_rows,
    };
    let d_message_search = MessageSearch::new(message_search_emit, model);
    d_message_search
}

#[no_mangle]
pub unsafe extern "C" fn message_search_free(ptr: *mut MessageSearch) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn message_search_clear_search(ptr: *mut MessageSearch) {
    let obj = &mut *ptr;
    obj.clear_search()
}

#[no_mangle]
pub unsafe extern "C" fn message_search_regex_search_get(
    ptr: *const MessageSearch
) -> COption<bool> {
    match (&*ptr).regex_search() {
        Some(value) => COption {
            data: value,
            some: true,
        },
        None => COption {
            data: bool::default(),
            some: false,
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_search_regex_search_set(
    ptr: *mut MessageSearch,
    value: Option<bool>,
) {
    (&mut *ptr).set_regex_search(value)
}

#[no_mangle]
pub unsafe extern "C" fn message_search_regex_search_set_none(ptr: *mut MessageSearch) {
    let obj = &mut *ptr;
    obj.set_regex_search(None);
}

#[no_mangle]
pub unsafe extern "C" fn message_search_search_pattern_get(
    ptr: *const MessageSearch,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.search_pattern();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_search_search_pattern_set(
    ptr: *mut MessageSearch,
    value: *const c_ushort,
    len: c_int,
) {
    let obj = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, value, len);
    obj.set_search_pattern(Some(s));
}

#[no_mangle]
pub unsafe extern "C" fn message_search_search_pattern_set_none(ptr: *mut MessageSearch) {
    let obj = &mut *ptr;
    obj.set_search_pattern(None);
}

#[no_mangle]
pub unsafe extern "C" fn message_search_row_count(ptr: *const MessageSearch) -> c_int {
    to_c_int((&*ptr).row_count())
}

#[no_mangle]
pub unsafe extern "C" fn message_search_insert_rows(
    ptr: *mut MessageSearch,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).insert_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_search_remove_rows(
    ptr: *mut MessageSearch,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).remove_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_search_can_fetch_more(ptr: *const MessageSearch) -> bool {
    (&*ptr).can_fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn message_search_fetch_more(ptr: *mut MessageSearch) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn message_search_sort(
    ptr: *mut MessageSearch,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn message_search_data_author(
    ptr: *const MessageSearch,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.author(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_search_data_body(
    ptr: *const MessageSearch,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.body(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_search_data_conversation(
    ptr: *const MessageSearch,
    row: c_int,
    d: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.conversation(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_search_data_conversation_color(
    ptr: *const MessageSearch,
    row: c_int,
) -> COption<u32> {
    let obj = &*ptr;
    obj.conversation_color(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn message_search_data_conversation_pairwise(
    ptr: *const MessageSearch,
    row: c_int,
) -> COption<bool> {
    let obj = &*ptr;
    obj.conversation_pairwise(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn message_search_data_conversation_picture(
    ptr: *const MessageSearch,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.conversation_picture(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_search_data_conversation_title(
    ptr: *const MessageSearch,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.conversation_title(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_search_data_has_attachments(
    ptr: *const MessageSearch,
    row: c_int,
) -> COption<bool> {
    let obj = &*ptr;
    obj.has_attachments(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn message_search_data_msg_id(
    ptr: *const MessageSearch,
    row: c_int,
    d: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.msg_id(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_search_data_time(
    ptr: *const MessageSearch,
    row: c_int,
) -> COption<i64> {
    let obj = &*ptr;
    obj.time(to_usize(row).unwrap_or(0)).into()
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MessageSearchPtrBundle {
    message_search: *mut MessageSearchQObject,
    message_search_regex_search_changed: fn(*mut MessageSearchQObject),
    message_search_search_pattern_changed: fn(*mut MessageSearchQObject),
    message_search_new_data_ready: fn(*mut MessageSearchQObject),
    message_search_layout_about_to_be_changed: fn(*mut MessageSearchQObject),
    message_search_layout_changed: fn(*mut MessageSearchQObject),
    message_search_data_changed: fn(*mut MessageSearchQObject, usize, usize),
    message_search_begin_reset_model: fn(*mut MessageSearchQObject),
    message_search_end_reset_model: fn(*mut MessageSearchQObject),
    message_search_begin_insert_rows: fn(*mut MessageSearchQObject, usize, usize),
    message_search_end_insert_rows: fn(*mut MessageSearchQObject),
    message_search_begin_move_rows: fn(*mut MessageSearchQObject, usize, usize, usize),
    message_search_end_move_rows: fn(*mut MessageSearchQObject),
    message_search_begin_remove_rows: fn(*mut MessageSearchQObject, usize, usize),
    message_search_end_remove_rows: fn(*mut MessageSearchQObject),
}
