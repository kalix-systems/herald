use super::*;

pub struct SharedConversationsQObject;

pub struct SharedConversationsEmitter {
    pub(super) qobject: Arc<AtomicPtr<SharedConversationsQObject>>,
    pub(super) user_id_changed: fn(*mut SharedConversationsQObject),
    pub(super) new_data_ready: fn(*mut SharedConversationsQObject),
    pub(super) try_load: fn(*mut SharedConversationsQObject),
}

impl SharedConversationsEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> SharedConversationsEmitter {
        SharedConversationsEmitter {
            qobject: self.qobject.clone(),
            user_id_changed: self.user_id_changed,
            try_load: self.try_load,
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const SharedConversationsQObject = null();
        self.qobject
            .store(n as *mut SharedConversationsQObject, Ordering::SeqCst);
    }

    pub fn user_id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.user_id_changed)(ptr);
        }
    }

    pub fn try_load(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.try_load)(ptr);
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
pub struct SharedConversationsList {
    pub(super) qobject: *mut SharedConversationsQObject,
    pub(super) layout_about_to_be_changed: fn(*mut SharedConversationsQObject),
    pub(super) layout_changed: fn(*mut SharedConversationsQObject),
    pub(super) begin_reset_model: fn(*mut SharedConversationsQObject),
    pub(super) end_reset_model: fn(*mut SharedConversationsQObject),
    pub(super) end_insert_rows: fn(*mut SharedConversationsQObject),
    pub(super) end_move_rows: fn(*mut SharedConversationsQObject),
    pub(super) end_remove_rows: fn(*mut SharedConversationsQObject),
    pub(super) begin_insert_rows: fn(*mut SharedConversationsQObject, usize, usize),
    pub(super) begin_remove_rows: fn(*mut SharedConversationsQObject, usize, usize),
    pub(super) data_changed: fn(*mut SharedConversationsQObject, usize, usize),
    pub(super) begin_move_rows: fn(*mut SharedConversationsQObject, usize, usize, usize),
}

impl SharedConversationsList {
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

pub trait SharedConversationsTrait {
    fn new(
        emit: SharedConversationsEmitter,
        model: SharedConversationsList,
    ) -> Self;

    fn emit(&mut self) -> &mut SharedConversationsEmitter;

    fn user_id(&self) -> Option<&str>;

    fn set_user_id(
        &mut self,
        value: Option<String>,
    );

    fn load(&mut self) -> ();

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

    fn conversation_id(
        &self,
        index: usize,
    ) -> &[u8];
}

#[no_mangle]
pub unsafe extern "C" fn shared_conversations_new(
    ptr_bundle: *mut SharedConversationsPtrBundle
) -> *mut SharedConversations {
    let d_shared_conversations = shared_conversations_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_shared_conversations))
}

pub unsafe fn shared_conversations_new_inner(
    ptr_bundle: *mut SharedConversationsPtrBundle
) -> SharedConversations {
    let ptr_bundle = *ptr_bundle;

    let SharedConversationsPtrBundle {
        shared_conversations,
        shared_conversations_user_id_changed,
        shared_conversations_new_data_ready,
        shared_conversations_layout_about_to_be_changed,
        shared_conversations_layout_changed,
        shared_conversations_data_changed,
        shared_conversations_begin_reset_model,
        shared_conversations_end_reset_model,
        shared_conversations_begin_insert_rows,
        shared_conversations_end_insert_rows,
        shared_conversations_begin_move_rows,
        shared_conversations_end_move_rows,
        shared_conversations_begin_remove_rows,
        shared_conversations_end_remove_rows,
        shared_conversations_try_load,
    } = ptr_bundle;
    let shared_conversations_emit = SharedConversationsEmitter {
        qobject: Arc::new(AtomicPtr::new(shared_conversations)),
        user_id_changed: shared_conversations_user_id_changed,
        new_data_ready: shared_conversations_new_data_ready,
        try_load: shared_conversations_try_load,
    };
    let model = SharedConversationsList {
        qobject: shared_conversations,
        layout_about_to_be_changed: shared_conversations_layout_about_to_be_changed,
        layout_changed: shared_conversations_layout_changed,
        data_changed: shared_conversations_data_changed,
        begin_reset_model: shared_conversations_begin_reset_model,
        end_reset_model: shared_conversations_end_reset_model,
        begin_insert_rows: shared_conversations_begin_insert_rows,
        end_insert_rows: shared_conversations_end_insert_rows,
        begin_move_rows: shared_conversations_begin_move_rows,
        end_move_rows: shared_conversations_end_move_rows,
        begin_remove_rows: shared_conversations_begin_remove_rows,
        end_remove_rows: shared_conversations_end_remove_rows,
    };
    let d_shared_conversations = SharedConversations::new(shared_conversations_emit, model);
    d_shared_conversations
}

#[no_mangle]
pub unsafe extern "C" fn shared_conversations_free(ptr: *mut SharedConversations) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn shared_conversations_load(ptr: *mut SharedConversations) {
    let obj = &mut *ptr;
    obj.load()
}

#[no_mangle]
pub unsafe extern "C" fn shared_conversations_user_id_get(
    ptr: *const SharedConversations,
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
pub unsafe extern "C" fn shared_conversations_user_id_set(
    ptr: *mut SharedConversations,
    value: *const c_ushort,
    len: c_int,
) {
    let obj = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, value, len);
    obj.set_user_id(Some(s));
}

#[no_mangle]
pub unsafe extern "C" fn shared_conversations_user_id_set_none(ptr: *mut SharedConversations) {
    let obj = &mut *ptr;
    obj.set_user_id(None);
}

#[no_mangle]
pub unsafe extern "C" fn shared_conversations_row_count(ptr: *const SharedConversations) -> c_int {
    to_c_int((&*ptr).row_count())
}

#[no_mangle]
pub unsafe extern "C" fn shared_conversations_insert_rows(
    ptr: *mut SharedConversations,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).insert_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn shared_conversations_remove_rows(
    ptr: *mut SharedConversations,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).remove_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn shared_conversations_can_fetch_more(
    ptr: *const SharedConversations
) -> bool {
    (&*ptr).can_fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn shared_conversations_fetch_more(ptr: *mut SharedConversations) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn shared_conversations_sort(
    ptr: *mut SharedConversations,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn shared_conversations_data_conversation_id(
    ptr: *const SharedConversations,
    row: c_int,
    d: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.conversation_id(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SharedConversationsPtrBundle {
    shared_conversations: *mut SharedConversationsQObject,
    shared_conversations_user_id_changed: fn(*mut SharedConversationsQObject),
    shared_conversations_new_data_ready: fn(*mut SharedConversationsQObject),
    shared_conversations_layout_about_to_be_changed: fn(*mut SharedConversationsQObject),
    shared_conversations_layout_changed: fn(*mut SharedConversationsQObject),
    shared_conversations_data_changed: fn(*mut SharedConversationsQObject, usize, usize),
    shared_conversations_begin_reset_model: fn(*mut SharedConversationsQObject),
    shared_conversations_end_reset_model: fn(*mut SharedConversationsQObject),
    shared_conversations_begin_insert_rows: fn(*mut SharedConversationsQObject, usize, usize),
    shared_conversations_end_insert_rows: fn(*mut SharedConversationsQObject),
    shared_conversations_begin_move_rows: fn(*mut SharedConversationsQObject, usize, usize, usize),
    shared_conversations_end_move_rows: fn(*mut SharedConversationsQObject),
    shared_conversations_begin_remove_rows: fn(*mut SharedConversationsQObject, usize, usize),
    shared_conversations_end_remove_rows: fn(*mut SharedConversationsQObject),
    shared_conversations_try_load: fn(*mut SharedConversationsQObject),
}
