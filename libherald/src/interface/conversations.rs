use super::*;

pub struct ConversationsQObject;

pub struct ConversationsEmitter {
    pub(super) qobject: Arc<AtomicPtr<ConversationsQObject>>,
    pub(super) filter_changed: fn(*mut ConversationsQObject),
    pub(super) filter_regex_changed: fn(*mut ConversationsQObject),
    pub(super) new_data_ready: fn(*mut ConversationsQObject),
}

impl ConversationsEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> ConversationsEmitter {
        ConversationsEmitter {
            qobject: self.qobject.clone(),
            filter_changed: self.filter_changed,
            filter_regex_changed: self.filter_regex_changed,
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const ConversationsQObject = null();
        self.qobject
            .store(n as *mut ConversationsQObject, Ordering::SeqCst);
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
pub struct ConversationsList {
    pub(super) qobject: *mut ConversationsQObject,
    pub(super) layout_about_to_be_changed: fn(*mut ConversationsQObject),
    pub(super) layout_changed: fn(*mut ConversationsQObject),
    pub(super) begin_reset_model: fn(*mut ConversationsQObject),
    pub(super) end_reset_model: fn(*mut ConversationsQObject),
    pub(super) end_insert_rows: fn(*mut ConversationsQObject),
    pub(super) end_move_rows: fn(*mut ConversationsQObject),
    pub(super) end_remove_rows: fn(*mut ConversationsQObject),
    pub(super) begin_insert_rows: fn(*mut ConversationsQObject, usize, usize),
    pub(super) begin_remove_rows: fn(*mut ConversationsQObject, usize, usize),
    pub(super) data_changed: fn(*mut ConversationsQObject, usize, usize),
    pub(super) begin_move_rows: fn(*mut ConversationsQObject, usize, usize, usize),
    pub(super) messages_ptr_bundle_factory: fn(*mut ConversationsQObject) -> *mut MessagesPtrBundle,
}

impl ConversationsList {
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

    pub fn messages_new(&mut self) -> Option<Messages> {
        if self.qobject.is_null() {
            return None;
        }
        let ptr_bundle = (self.messages_ptr_bundle_factory)(self.qobject);
        Some(unsafe { messages_new_inner(ptr_bundle) })
    }
}

pub trait ConversationsTrait {
    fn new(
        emit: ConversationsEmitter,
        model: ConversationsList,
    ) -> Self;

    fn emit(&mut self) -> &mut ConversationsEmitter;

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

    fn clear_filter(&mut self) -> ();

    fn index_by_id(
        &self,
        conversation_id: &[u8],
    ) -> u64;

    fn remove_conversation(
        &mut self,
        row_index: u64,
    ) -> bool;

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

    fn conversation_id(
        &self,
        index: usize,
    ) -> Vec<u8>;

    fn expiration_period(
        &self,
        index: usize,
    ) -> u8;

    fn set_expiration_period(
        &mut self,
        index: usize,
        _: u8,
    ) -> bool;

    fn matched(
        &self,
        index: usize,
    ) -> bool;

    fn messages(
        &self,
        index: usize,
    ) -> &Messages;

    fn messages_mut(
        &mut self,
        index: usize,
    ) -> &mut Messages;

    fn muted(
        &self,
        index: usize,
    ) -> bool;

    fn set_muted(
        &mut self,
        index: usize,
        _: bool,
    ) -> bool;

    fn pairwise(
        &self,
        index: usize,
    ) -> bool;

    fn picture(
        &self,
        index: usize,
    ) -> Option<String>;

    fn set_picture(
        &mut self,
        index: usize,
        _: Option<String>,
    ) -> bool;

    fn title(
        &self,
        index: usize,
    ) -> Option<String>;

    fn set_title(
        &mut self,
        index: usize,
        _: Option<String>,
    ) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn conversations_new(
    ptr_bundle: *mut ConversationsPtrBundle
) -> *mut Conversations {
    let d_conversations = conversations_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_conversations))
}

pub unsafe fn conversations_new_inner(ptr_bundle: *mut ConversationsPtrBundle) -> Conversations {
    let ptr_bundle = *ptr_bundle;

    let ConversationsPtrBundle {
        conversations,
        conversations_filter_changed,
        conversations_filter_regex_changed,
        conversations_new_data_ready,
        conversations_layout_about_to_be_changed,
        conversations_layout_changed,
        conversations_data_changed,
        conversations_begin_reset_model,
        conversations_end_reset_model,
        conversations_begin_insert_rows,
        conversations_end_insert_rows,
        conversations_begin_move_rows,
        conversations_end_move_rows,
        conversations_begin_remove_rows,
        conversations_end_remove_rows,
        messages_ptr_bundle_factory,
    } = ptr_bundle;
    let conversations_emit = ConversationsEmitter {
        qobject: Arc::new(AtomicPtr::new(conversations)),
        filter_changed: conversations_filter_changed,
        filter_regex_changed: conversations_filter_regex_changed,
        new_data_ready: conversations_new_data_ready,
    };
    let model = ConversationsList {
        qobject: conversations,
        layout_about_to_be_changed: conversations_layout_about_to_be_changed,
        layout_changed: conversations_layout_changed,
        data_changed: conversations_data_changed,
        begin_reset_model: conversations_begin_reset_model,
        end_reset_model: conversations_end_reset_model,
        begin_insert_rows: conversations_begin_insert_rows,
        end_insert_rows: conversations_end_insert_rows,
        begin_move_rows: conversations_begin_move_rows,
        end_move_rows: conversations_end_move_rows,
        begin_remove_rows: conversations_begin_remove_rows,
        end_remove_rows: conversations_end_remove_rows,

        messages_ptr_bundle_factory,
    };
    let d_conversations = Conversations::new(conversations_emit, model);
    d_conversations
}

#[no_mangle]
pub unsafe extern "C" fn conversations_free(ptr: *mut Conversations) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn conversations_clear_filter(ptr: *mut Conversations) {
    let obj = &mut *ptr;
    obj.clear_filter()
}

#[no_mangle]
pub unsafe extern "C" fn conversations_index_by_id(
    ptr: *const Conversations,
    conversation_id_str: *const c_char,
    conversation_id_len: c_int,
) -> u64 {
    let obj = &*ptr;
    let conversation_id = { qba_slice!(conversation_id_str, conversation_id_len) };
    obj.index_by_id(conversation_id)
}

#[no_mangle]
pub unsafe extern "C" fn conversations_remove_conversation(
    ptr: *mut Conversations,
    row_index: u64,
) -> bool {
    let obj = &mut *ptr;
    obj.remove_conversation(row_index)
}

#[no_mangle]
pub unsafe extern "C" fn conversations_toggle_filter_regex(ptr: *mut Conversations) -> bool {
    let obj = &mut *ptr;
    obj.toggle_filter_regex()
}

#[no_mangle]
pub unsafe extern "C" fn conversations_filter_get(
    ptr: *const Conversations,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.filter();
    let str_: *const c_char = value.as_ptr() as *const c_char;
    set(prop, str_, to_c_int(value.len()));
}

#[no_mangle]
pub unsafe extern "C" fn conversations_filter_set(
    ptr: *mut Conversations,
    value: *const c_ushort,
    len: c_int,
) {
    let obj = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, value, len);
    obj.set_filter(s);
}

#[no_mangle]
pub unsafe extern "C" fn conversations_filter_regex_get(ptr: *const Conversations) -> bool {
    (&*ptr).filter_regex()
}

#[no_mangle]
pub unsafe extern "C" fn conversations_filter_regex_set(
    ptr: *mut Conversations,
    value: bool,
) {
    (&mut *ptr).set_filter_regex(value)
}

#[no_mangle]
pub unsafe extern "C" fn conversations_row_count(ptr: *const Conversations) -> c_int {
    to_c_int((&*ptr).row_count())
}

#[no_mangle]
pub unsafe extern "C" fn conversations_insert_rows(
    ptr: *mut Conversations,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).insert_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn conversations_remove_rows(
    ptr: *mut Conversations,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).remove_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn conversations_can_fetch_more(ptr: *const Conversations) -> bool {
    (&*ptr).can_fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn conversations_fetch_more(ptr: *mut Conversations) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn conversations_sort(
    ptr: *mut Conversations,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn conversations_data_color(
    ptr: *const Conversations,
    row: c_int,
) -> u32 {
    let obj = &*ptr;
    obj.color(to_usize(row).unwrap_or(0))
}

#[no_mangle]
pub unsafe extern "C" fn conversations_set_data_color(
    ptr: *mut Conversations,
    row: c_int,
    value: u32,
) -> bool {
    (&mut *ptr).set_color(to_usize(row).unwrap_or(0), value)
}

#[no_mangle]
pub unsafe extern "C" fn conversations_data_conversation_id(
    ptr: *const Conversations,
    row: c_int,
    d: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.conversation_id(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn conversations_data_expiration_period(
    ptr: *const Conversations,
    row: c_int,
) -> u8 {
    let obj = &*ptr;
    obj.expiration_period(to_usize(row).unwrap_or(0))
}

#[no_mangle]
pub unsafe extern "C" fn conversations_set_data_expiration_period(
    ptr: *mut Conversations,
    row: c_int,
    value: u8,
) -> bool {
    (&mut *ptr).set_expiration_period(to_usize(row).unwrap_or(0), value)
}

#[no_mangle]
pub unsafe extern "C" fn conversations_data_matched(
    ptr: *const Conversations,
    row: c_int,
) -> bool {
    let obj = &*ptr;
    obj.matched(to_usize(row).unwrap_or(0))
}

#[no_mangle]
pub unsafe extern "C" fn conversations_data_messages(
    ptr: *mut Conversations,
    row: c_int,
) -> *mut Messages {
    (&mut *ptr).messages_mut(to_usize(row).unwrap_or(0))
}

#[no_mangle]
pub unsafe extern "C" fn conversations_data_muted(
    ptr: *const Conversations,
    row: c_int,
) -> bool {
    let obj = &*ptr;
    obj.muted(to_usize(row).unwrap_or(0))
}

#[no_mangle]
pub unsafe extern "C" fn conversations_set_data_muted(
    ptr: *mut Conversations,
    row: c_int,
    value: bool,
) -> bool {
    (&mut *ptr).set_muted(to_usize(row).unwrap_or(0), value)
}

#[no_mangle]
pub unsafe extern "C" fn conversations_data_pairwise(
    ptr: *const Conversations,
    row: c_int,
) -> bool {
    let obj = &*ptr;
    obj.pairwise(to_usize(row).unwrap_or(0))
}

#[no_mangle]
pub unsafe extern "C" fn conversations_data_picture(
    ptr: *const Conversations,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.picture(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn conversations_set_data_picture(
    ptr: *mut Conversations,
    row: c_int,
    str_: *const c_ushort,
    len: c_int,
) -> bool {
    let obj = &mut *ptr;
    let mut value = String::new();
    set_string_from_utf16(&mut value, str_, len);
    obj.set_picture(to_usize(row).unwrap_or(0), Some(value))
}

#[no_mangle]
pub unsafe extern "C" fn conversations_set_data_picture_none(
    ptr: *mut Conversations,
    row: c_int,
) -> bool {
    (&mut *ptr).set_picture(to_usize(row).unwrap_or(0), None)
}

#[no_mangle]
pub unsafe extern "C" fn conversations_data_title(
    ptr: *const Conversations,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.title(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn conversations_set_data_title(
    ptr: *mut Conversations,
    row: c_int,
    str_: *const c_ushort,
    len: c_int,
) -> bool {
    let obj = &mut *ptr;
    let mut value = String::new();
    set_string_from_utf16(&mut value, str_, len);
    obj.set_title(to_usize(row).unwrap_or(0), Some(value))
}

#[no_mangle]
pub unsafe extern "C" fn conversations_set_data_title_none(
    ptr: *mut Conversations,
    row: c_int,
) -> bool {
    (&mut *ptr).set_title(to_usize(row).unwrap_or(0), None)
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ConversationsPtrBundle {
    conversations: *mut ConversationsQObject,
    conversations_filter_changed: fn(*mut ConversationsQObject),
    conversations_filter_regex_changed: fn(*mut ConversationsQObject),
    conversations_new_data_ready: fn(*mut ConversationsQObject),
    conversations_layout_about_to_be_changed: fn(*mut ConversationsQObject),
    conversations_layout_changed: fn(*mut ConversationsQObject),
    conversations_data_changed: fn(*mut ConversationsQObject, usize, usize),
    conversations_begin_reset_model: fn(*mut ConversationsQObject),
    conversations_end_reset_model: fn(*mut ConversationsQObject),
    conversations_begin_insert_rows: fn(*mut ConversationsQObject, usize, usize),
    conversations_end_insert_rows: fn(*mut ConversationsQObject),
    conversations_begin_move_rows: fn(*mut ConversationsQObject, usize, usize, usize),
    conversations_end_move_rows: fn(*mut ConversationsQObject),
    conversations_begin_remove_rows: fn(*mut ConversationsQObject, usize, usize),
    conversations_end_remove_rows: fn(*mut ConversationsQObject),
    messages_ptr_bundle_factory: fn(*mut ConversationsQObject) -> *mut MessagesPtrBundle,
}
