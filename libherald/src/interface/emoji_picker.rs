use super::*;

pub struct EmojiPickerQObject;

pub struct EmojiPickerEmitter {
    pub(super) qobject: Arc<AtomicPtr<EmojiPickerQObject>>,
    pub(super) search_string_changed: fn(*mut EmojiPickerQObject),
    pub(super) new_data_ready: fn(*mut EmojiPickerQObject),
}

impl EmojiPickerEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> EmojiPickerEmitter {
        EmojiPickerEmitter {
            qobject: self.qobject.clone(),
            search_string_changed: self.search_string_changed,
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const EmojiPickerQObject = null();
        self.qobject
            .store(n as *mut EmojiPickerQObject, Ordering::SeqCst);
    }

    pub fn search_string_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.search_string_changed)(ptr);
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
pub struct EmojiPickerList {
    pub(super) qobject: *mut EmojiPickerQObject,
    pub(super) layout_about_to_be_changed: fn(*mut EmojiPickerQObject),
    pub(super) layout_changed: fn(*mut EmojiPickerQObject),
    pub(super) begin_reset_model: fn(*mut EmojiPickerQObject),
    pub(super) end_reset_model: fn(*mut EmojiPickerQObject),
    pub(super) end_insert_rows: fn(*mut EmojiPickerQObject),
    pub(super) end_move_rows: fn(*mut EmojiPickerQObject),
    pub(super) end_remove_rows: fn(*mut EmojiPickerQObject),
    pub(super) begin_insert_rows: fn(*mut EmojiPickerQObject, usize, usize),
    pub(super) begin_remove_rows: fn(*mut EmojiPickerQObject, usize, usize),
    pub(super) data_changed: fn(*mut EmojiPickerQObject, usize, usize),
    pub(super) begin_move_rows: fn(*mut EmojiPickerQObject, usize, usize, usize),
}

impl EmojiPickerList {
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

pub trait EmojiPickerTrait {
    fn new(
        emit: EmojiPickerEmitter,
        model: EmojiPickerList,
    ) -> Self;

    fn emit(&mut self) -> &mut EmojiPickerEmitter;

    fn search_string(&self) -> Option<&str>;

    fn set_search_string(
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

    fn emoji(
        &self,
        index: usize,
    ) -> String;
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_new(
    ptr_bundle: *mut EmojiPickerPtrBundle
) -> *mut EmojiPicker {
    let d_emoji_picker = emoji_picker_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_emoji_picker))
}

pub unsafe fn emoji_picker_new_inner(ptr_bundle: *mut EmojiPickerPtrBundle) -> EmojiPicker {
    let ptr_bundle = *ptr_bundle;

    let EmojiPickerPtrBundle {
        emoji_picker,
        emoji_picker_search_string_changed,
        emoji_picker_new_data_ready,
        emoji_picker_layout_about_to_be_changed,
        emoji_picker_layout_changed,
        emoji_picker_data_changed,
        emoji_picker_begin_reset_model,
        emoji_picker_end_reset_model,
        emoji_picker_begin_insert_rows,
        emoji_picker_end_insert_rows,
        emoji_picker_begin_move_rows,
        emoji_picker_end_move_rows,
        emoji_picker_begin_remove_rows,
        emoji_picker_end_remove_rows,
    } = ptr_bundle;
    let emoji_picker_emit = EmojiPickerEmitter {
        qobject: Arc::new(AtomicPtr::new(emoji_picker)),
        search_string_changed: emoji_picker_search_string_changed,
        new_data_ready: emoji_picker_new_data_ready,
    };
    let model = EmojiPickerList {
        qobject: emoji_picker,
        layout_about_to_be_changed: emoji_picker_layout_about_to_be_changed,
        layout_changed: emoji_picker_layout_changed,
        data_changed: emoji_picker_data_changed,
        begin_reset_model: emoji_picker_begin_reset_model,
        end_reset_model: emoji_picker_end_reset_model,
        begin_insert_rows: emoji_picker_begin_insert_rows,
        end_insert_rows: emoji_picker_end_insert_rows,
        begin_move_rows: emoji_picker_begin_move_rows,
        end_move_rows: emoji_picker_end_move_rows,
        begin_remove_rows: emoji_picker_begin_remove_rows,
        end_remove_rows: emoji_picker_end_remove_rows,
    };
    let d_emoji_picker = EmojiPicker::new(emoji_picker_emit, model);
    d_emoji_picker
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_free(ptr: *mut EmojiPicker) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_clear_search(ptr: *mut EmojiPicker) {
    let obj = &mut *ptr;
    obj.clear_search()
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_search_string_get(
    ptr: *const EmojiPicker,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.search_string();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_search_string_set(
    ptr: *mut EmojiPicker,
    value: *const c_ushort,
    len: c_int,
) {
    let obj = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, value, len);
    obj.set_search_string(Some(s));
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_search_string_set_none(ptr: *mut EmojiPicker) {
    let obj = &mut *ptr;
    obj.set_search_string(None);
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_row_count(ptr: *const EmojiPicker) -> c_int {
    to_c_int((&*ptr).row_count())
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_insert_rows(
    ptr: *mut EmojiPicker,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).insert_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_remove_rows(
    ptr: *mut EmojiPicker,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).remove_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_can_fetch_more(ptr: *const EmojiPicker) -> bool {
    (&*ptr).can_fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_fetch_more(ptr: *mut EmojiPicker) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_sort(
    ptr: *mut EmojiPicker,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_data_emoji(
    ptr: *const EmojiPicker,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.emoji(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EmojiPickerPtrBundle {
    emoji_picker: *mut EmojiPickerQObject,
    emoji_picker_search_string_changed: fn(*mut EmojiPickerQObject),
    emoji_picker_new_data_ready: fn(*mut EmojiPickerQObject),
    emoji_picker_layout_about_to_be_changed: fn(*mut EmojiPickerQObject),
    emoji_picker_layout_changed: fn(*mut EmojiPickerQObject),
    emoji_picker_data_changed: fn(*mut EmojiPickerQObject, usize, usize),
    emoji_picker_begin_reset_model: fn(*mut EmojiPickerQObject),
    emoji_picker_end_reset_model: fn(*mut EmojiPickerQObject),
    emoji_picker_begin_insert_rows: fn(*mut EmojiPickerQObject, usize, usize),
    emoji_picker_end_insert_rows: fn(*mut EmojiPickerQObject),
    emoji_picker_begin_move_rows: fn(*mut EmojiPickerQObject, usize, usize, usize),
    emoji_picker_end_move_rows: fn(*mut EmojiPickerQObject),
    emoji_picker_begin_remove_rows: fn(*mut EmojiPickerQObject, usize, usize),
    emoji_picker_end_remove_rows: fn(*mut EmojiPickerQObject),
}
