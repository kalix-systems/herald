use super::*;

pub struct EmojiPickerQObject;

pub struct EmojiPickerEmitter {
    pub(super) qobject: Arc<AtomicPtr<EmojiPickerQObject>>,
    pub(super) activities_index_changed: fn(*mut EmojiPickerQObject),
    pub(super) body_index_changed: fn(*mut EmojiPickerQObject),
    pub(super) flags_index_changed: fn(*mut EmojiPickerQObject),
    pub(super) food_index_changed: fn(*mut EmojiPickerQObject),
    pub(super) locations_index_changed: fn(*mut EmojiPickerQObject),
    pub(super) nature_index_changed: fn(*mut EmojiPickerQObject),
    pub(super) objects_index_changed: fn(*mut EmojiPickerQObject),
    pub(super) smileys_index_changed: fn(*mut EmojiPickerQObject),
    pub(super) symbols_index_changed: fn(*mut EmojiPickerQObject),
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
            activities_index_changed: self.activities_index_changed,
            body_index_changed: self.body_index_changed,
            flags_index_changed: self.flags_index_changed,
            food_index_changed: self.food_index_changed,
            locations_index_changed: self.locations_index_changed,
            nature_index_changed: self.nature_index_changed,
            objects_index_changed: self.objects_index_changed,
            smileys_index_changed: self.smileys_index_changed,
            symbols_index_changed: self.symbols_index_changed,
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const EmojiPickerQObject = null();
        self.qobject
            .store(n as *mut EmojiPickerQObject, Ordering::SeqCst);
    }

    pub fn activities_index_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.activities_index_changed)(ptr);
        }
    }

    pub fn body_index_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.body_index_changed)(ptr);
        }
    }

    pub fn flags_index_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.flags_index_changed)(ptr);
        }
    }

    pub fn food_index_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.food_index_changed)(ptr);
        }
    }

    pub fn locations_index_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.locations_index_changed)(ptr);
        }
    }

    pub fn nature_index_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.nature_index_changed)(ptr);
        }
    }

    pub fn objects_index_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.objects_index_changed)(ptr);
        }
    }

    pub fn smileys_index_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.smileys_index_changed)(ptr);
        }
    }

    pub fn symbols_index_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.symbols_index_changed)(ptr);
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

    fn activities_index(&self) -> u32;

    fn body_index(&self) -> u32;

    fn flags_index(&self) -> u32;

    fn food_index(&self) -> u32;

    fn locations_index(&self) -> u32;

    fn nature_index(&self) -> u32;

    fn objects_index(&self) -> u32;

    fn smileys_index(&self) -> u32;

    fn symbols_index(&self) -> u32;

    fn clear_search(&mut self) -> ();

    fn set_search_string(
        &mut self,
        search_string: String,
    ) -> ();

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
    ) -> &str;

    fn skintone_modifier(
        &self,
        index: usize,
    ) -> bool;
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
        emoji_picker_activities_index_changed,
        emoji_picker_body_index_changed,
        emoji_picker_flags_index_changed,
        emoji_picker_food_index_changed,
        emoji_picker_locations_index_changed,
        emoji_picker_nature_index_changed,
        emoji_picker_objects_index_changed,
        emoji_picker_smileys_index_changed,
        emoji_picker_symbols_index_changed,
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
        activities_index_changed: emoji_picker_activities_index_changed,
        body_index_changed: emoji_picker_body_index_changed,
        flags_index_changed: emoji_picker_flags_index_changed,
        food_index_changed: emoji_picker_food_index_changed,
        locations_index_changed: emoji_picker_locations_index_changed,
        nature_index_changed: emoji_picker_nature_index_changed,
        objects_index_changed: emoji_picker_objects_index_changed,
        smileys_index_changed: emoji_picker_smileys_index_changed,
        symbols_index_changed: emoji_picker_symbols_index_changed,
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
pub unsafe extern "C" fn emoji_picker_set_search_string(
    ptr: *mut EmojiPicker,
    search_string_str: *const c_ushort,
    search_string_len: c_int,
) {
    let obj = &mut *ptr;
    let mut search_string = String::new();
    set_string_from_utf16(&mut search_string, search_string_str, search_string_len);
    obj.set_search_string(search_string)
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_activities_index_get(ptr: *const EmojiPicker) -> u32 {
    (&*ptr).activities_index()
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_body_index_get(ptr: *const EmojiPicker) -> u32 {
    (&*ptr).body_index()
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_flags_index_get(ptr: *const EmojiPicker) -> u32 {
    (&*ptr).flags_index()
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_food_index_get(ptr: *const EmojiPicker) -> u32 {
    (&*ptr).food_index()
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_locations_index_get(ptr: *const EmojiPicker) -> u32 {
    (&*ptr).locations_index()
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_nature_index_get(ptr: *const EmojiPicker) -> u32 {
    (&*ptr).nature_index()
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_objects_index_get(ptr: *const EmojiPicker) -> u32 {
    (&*ptr).objects_index()
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_smileys_index_get(ptr: *const EmojiPicker) -> u32 {
    (&*ptr).smileys_index()
}

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_symbols_index_get(ptr: *const EmojiPicker) -> u32 {
    (&*ptr).symbols_index()
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

#[no_mangle]
pub unsafe extern "C" fn emoji_picker_data_skintone_modifier(
    ptr: *const EmojiPicker,
    row: c_int,
) -> bool {
    let obj = &*ptr;
    obj.skintone_modifier(to_usize(row).unwrap_or(0))
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EmojiPickerPtrBundle {
    emoji_picker: *mut EmojiPickerQObject,
    emoji_picker_activities_index_changed: fn(*mut EmojiPickerQObject),
    emoji_picker_body_index_changed: fn(*mut EmojiPickerQObject),
    emoji_picker_flags_index_changed: fn(*mut EmojiPickerQObject),
    emoji_picker_food_index_changed: fn(*mut EmojiPickerQObject),
    emoji_picker_locations_index_changed: fn(*mut EmojiPickerQObject),
    emoji_picker_nature_index_changed: fn(*mut EmojiPickerQObject),
    emoji_picker_objects_index_changed: fn(*mut EmojiPickerQObject),
    emoji_picker_smileys_index_changed: fn(*mut EmojiPickerQObject),
    emoji_picker_symbols_index_changed: fn(*mut EmojiPickerQObject),
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
