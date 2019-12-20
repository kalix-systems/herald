use super::*;

pub struct ConversationBuilderQObject;

pub struct ConversationBuilderEmitter {
    pub(super) qobject: Arc<AtomicPtr<ConversationBuilderQObject>>,
    pub(super) picture_changed: fn(*mut ConversationBuilderQObject),
    pub(super) new_data_ready: fn(*mut ConversationBuilderQObject),
}

impl ConversationBuilderEmitter {
    /// Clone the emitter
    /// 
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> ConversationBuilderEmitter {
        ConversationBuilderEmitter {
            qobject: self.qobject.clone(),
            picture_changed: self.picture_changed,
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const ConversationBuilderQObject = null();
        self.qobject.store(n as *mut ConversationBuilderQObject, Ordering::SeqCst);
    }

    pub fn picture_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.picture_changed)(ptr);
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
pub struct ConversationBuilderList {
    pub(super) qobject: *mut ConversationBuilderQObject,
    pub(super) layout_about_to_be_changed: fn(*mut ConversationBuilderQObject),
    pub(super) layout_changed: fn(*mut ConversationBuilderQObject),
    pub(super) begin_reset_model: fn(*mut ConversationBuilderQObject),
    pub(super) end_reset_model: fn(*mut ConversationBuilderQObject),
    pub(super) end_insert_rows: fn(*mut ConversationBuilderQObject),
    pub(super) end_move_rows: fn(*mut ConversationBuilderQObject),
    pub(super) end_remove_rows: fn(*mut ConversationBuilderQObject),
    pub(super) begin_insert_rows: fn(*mut ConversationBuilderQObject,  usize, usize),
    pub(super) begin_remove_rows: fn(*mut ConversationBuilderQObject,  usize, usize),
    pub(super) data_changed: fn(*mut ConversationBuilderQObject, usize, usize),
    pub(super) begin_move_rows: fn(*mut ConversationBuilderQObject, usize, usize, usize),
}

impl ConversationBuilderList {
    pub fn layout_about_to_be_changed(&mut self) {
        if !self.qobject.is_null() { (self.layout_about_to_be_changed)(self.qobject); }
    }

    pub fn layout_changed(&mut self) {
        if !self.qobject.is_null() { (self.layout_changed)(self.qobject) }
    }

    pub fn begin_reset_model(&mut self) {
        if !self.qobject.is_null() { (self.begin_reset_model)(self.qobject); }
    }

    pub fn end_reset_model(&mut self) {
        if !self.qobject.is_null() { (self.end_reset_model)(self.qobject); }
    }

    pub fn end_insert_rows(&mut self) {
        if !self.qobject.is_null() { (self.end_insert_rows)(self.qobject); }
    }

    pub fn end_move_rows(&mut self) {
        if !self.qobject.is_null() { (self.end_move_rows)(self.qobject); }
    }

    pub fn end_remove_rows(&mut self) {
        if !self.qobject.is_null() { (self.end_remove_rows)(self.qobject); }
    }

    pub fn begin_insert_rows(&mut self, first: usize, last: usize) {
        if !self.qobject.is_null() { (self.begin_insert_rows)(self.qobject, first, last); }
    }

    pub fn begin_remove_rows(&mut self, first: usize, last: usize) {
        if !self.qobject.is_null() { (self.begin_remove_rows)(self.qobject, first, last); }
    }

    pub fn data_changed(&mut self, first: usize, last: usize) {
        if !self.qobject.is_null() { (self.data_changed)(self.qobject, first, last); }
    }

    pub fn begin_move_rows(&mut self, first: usize, last: usize, destination: usize) {
        if !self.qobject.is_null() { (self.begin_move_rows)(self.qobject, first, last, destination); }
    }
}

pub trait ConversationBuilderTrait {
    fn new(emit: ConversationBuilderEmitter, model: ConversationBuilderList) -> Self;

    fn emit(&mut self) -> &mut ConversationBuilderEmitter;

    fn picture(&self) -> Option<&str>;

    fn set_picture(&mut self, value: Option<String>);

    fn add_member(&mut self, user_id: String) -> bool;

    fn clear(&mut self) -> ();

    fn finalize(&mut self) -> ();

    fn remove_last(&mut self) -> ();

    fn remove_member_by_id(&mut self, user_id: String) -> bool;

    fn remove_member_by_index(&mut self, index: u64) -> bool;

    fn set_title(&mut self, title: String) -> ();

    fn row_count(&self) -> usize;

    fn insert_rows(&mut self, _row: usize, _count: usize) -> bool {
        false
    }

    fn remove_rows(&mut self, _row: usize, _count: usize) -> bool {
        false
    }

    fn can_fetch_more(&self) -> bool {
        false
    }

    fn fetch_more(&mut self) {

    }

    fn sort(&mut self, _: u8, _: SortOrder) {

    }

    fn member_id(&self, index: usize) -> &str;
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_new(ptr_bundle: *mut ConversationBuilderPtrBundle) -> *mut ConversationBuilder {
    let d_conversation_builder = conversation_builder_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_conversation_builder))
}

pub unsafe fn conversation_builder_new_inner(ptr_bundle: *mut ConversationBuilderPtrBundle) -> ConversationBuilder {
    let ptr_bundle = *ptr_bundle;

    let ConversationBuilderPtrBundle {
        conversation_builder
        ,
        conversation_builder_picture_changed,
        conversation_builder_new_data_ready,
        conversation_builder_layout_about_to_be_changed,
        conversation_builder_layout_changed,
        conversation_builder_data_changed,
        conversation_builder_begin_reset_model,
        conversation_builder_end_reset_model,
        conversation_builder_begin_insert_rows,
        conversation_builder_end_insert_rows,
        conversation_builder_begin_move_rows,
        conversation_builder_end_move_rows,
        conversation_builder_begin_remove_rows,
        conversation_builder_end_remove_rows,
    } = ptr_bundle;
    let conversation_builder_emit = ConversationBuilderEmitter {
        qobject: Arc::new(AtomicPtr::new(conversation_builder)),
        picture_changed: conversation_builder_picture_changed,
        new_data_ready: conversation_builder_new_data_ready,
    };
    let model = ConversationBuilderList {

                qobject: conversation_builder,
                layout_about_to_be_changed: conversation_builder_layout_about_to_be_changed,
                layout_changed: conversation_builder_layout_changed,
                data_changed: conversation_builder_data_changed,
                begin_reset_model: conversation_builder_begin_reset_model,
                end_reset_model: conversation_builder_end_reset_model,
                begin_insert_rows: conversation_builder_begin_insert_rows,
                end_insert_rows: conversation_builder_end_insert_rows,
                begin_move_rows: conversation_builder_begin_move_rows,
                end_move_rows: conversation_builder_end_move_rows,
                begin_remove_rows: conversation_builder_begin_remove_rows,
                end_remove_rows: conversation_builder_end_remove_rows,
                
    };
    let d_conversation_builder = ConversationBuilder::new(conversation_builder_emit, model
    );
    d_conversation_builder
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_free(ptr: *mut ConversationBuilder) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_add_member(ptr: *mut ConversationBuilder, user_id_str: *const c_ushort, user_id_len: c_int) -> bool {
    let obj = &mut *ptr;
    let mut user_id = String::new();
    set_string_from_utf16(&mut user_id, user_id_str, user_id_len);
    obj.add_member(
    user_id,
    )
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_clear(ptr: *mut ConversationBuilder) {
    let obj = &mut *ptr;
    obj.clear(
    )
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_finalize(ptr: *mut ConversationBuilder) {
    let obj = &mut *ptr;
    obj.finalize(
    )
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_remove_last(ptr: *mut ConversationBuilder) {
    let obj = &mut *ptr;
    obj.remove_last(
    )
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_remove_member_by_id(ptr: *mut ConversationBuilder, user_id_str: *const c_ushort, user_id_len: c_int) -> bool {
    let obj = &mut *ptr;
    let mut user_id = String::new();
    set_string_from_utf16(&mut user_id, user_id_str, user_id_len);
    obj.remove_member_by_id(
    user_id,
    )
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_remove_member_by_index(ptr: *mut ConversationBuilder, index: u64) -> bool {
    let obj = &mut *ptr;
    obj.remove_member_by_index(
    index,
    )
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_set_title(ptr: *mut ConversationBuilder, title_str: *const c_ushort, title_len: c_int) {
    let obj = &mut *ptr;
    let mut title = String::new();
    set_string_from_utf16(&mut title, title_str, title_len);
    obj.set_title(
    title,
    )
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_picture_get(ptr: *const ConversationBuilder, prop: *mut QString, set: fn(*mut QString, *const c_char, c_int)) {
    let obj = &*ptr;
    let value = obj.picture();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_picture_set(ptr: *mut ConversationBuilder, value: *const c_ushort, len: c_int) {
    let obj = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, value, len);
    obj.set_picture(Some(s));
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_picture_set_none(ptr: *mut ConversationBuilder) {
    let obj = &mut *ptr;
    obj.set_picture(None);
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_row_count(ptr: *const ConversationBuilder) -> c_int {
    to_c_int((&*ptr).row_count())
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_insert_rows(ptr: *mut ConversationBuilder, row: c_int, count: c_int) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => 
        {
            (&mut *ptr).insert_rows(row, count)
        }
        _ => false
    }
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_remove_rows(ptr: *mut ConversationBuilder, row: c_int, count: c_int) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => 
        {
            (&mut *ptr).remove_rows(row, count)
        }
        _ => false
    }
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_can_fetch_more(ptr: *const ConversationBuilder) -> bool {
    (&*ptr).can_fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_fetch_more(ptr: *mut ConversationBuilder) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_sort(ptr: *mut ConversationBuilder, column: u8, order: SortOrder) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn conversation_builder_data_member_id(ptr: *const ConversationBuilder, row: c_int, d: *mut QString, set: fn(*mut QString, *const c_char, len: c_int)) {
    let obj = &*ptr;
    let data = obj.member_id(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ConversationBuilderPtrBundle {
    conversation_builder: *mut ConversationBuilderQObject,
    conversation_builder_picture_changed: fn(*mut ConversationBuilderQObject),
    conversation_builder_new_data_ready: fn(*mut ConversationBuilderQObject),
    conversation_builder_layout_about_to_be_changed: fn(*mut ConversationBuilderQObject),
    conversation_builder_layout_changed: fn(*mut ConversationBuilderQObject),
    conversation_builder_data_changed: fn(*mut ConversationBuilderQObject, usize, usize),
    conversation_builder_begin_reset_model: fn(*mut ConversationBuilderQObject),
    conversation_builder_end_reset_model: fn(*mut ConversationBuilderQObject),
    conversation_builder_begin_insert_rows: fn(*mut ConversationBuilderQObject, usize, usize),
    conversation_builder_end_insert_rows: fn(*mut ConversationBuilderQObject),
    conversation_builder_begin_move_rows: fn(*mut ConversationBuilderQObject, usize, usize, usize),
    conversation_builder_end_move_rows: fn(*mut ConversationBuilderQObject),
    conversation_builder_begin_remove_rows: fn(*mut ConversationBuilderQObject, usize, usize),
    conversation_builder_end_remove_rows: fn(*mut ConversationBuilderQObject),
}
