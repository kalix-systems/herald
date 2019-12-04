use super::*;

pub struct MessageBuilderQObject;

pub struct MessageBuilderEmitter {
    pub(super) qobject: Arc<AtomicPtr<MessageBuilderQObject>>,
    pub(super) body_changed: fn(*mut MessageBuilderQObject),
    pub(super) has_attachments_changed: fn(*mut MessageBuilderQObject),
    pub(super) is_reply_changed: fn(*mut MessageBuilderQObject),
    pub(super) op_author_changed: fn(*mut MessageBuilderQObject),
    pub(super) op_body_changed: fn(*mut MessageBuilderQObject),
    pub(super) op_has_attachments_changed: fn(*mut MessageBuilderQObject),
    pub(super) op_id_changed: fn(*mut MessageBuilderQObject),
    pub(super) op_time_changed: fn(*mut MessageBuilderQObject),
    pub(super) new_data_ready: fn(*mut MessageBuilderQObject),
}

impl MessageBuilderEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> MessageBuilderEmitter {
        MessageBuilderEmitter {
            qobject: self.qobject.clone(),
            body_changed: self.body_changed,
            has_attachments_changed: self.has_attachments_changed,
            is_reply_changed: self.is_reply_changed,
            op_author_changed: self.op_author_changed,
            op_body_changed: self.op_body_changed,
            op_has_attachments_changed: self.op_has_attachments_changed,
            op_id_changed: self.op_id_changed,
            op_time_changed: self.op_time_changed,
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const MessageBuilderQObject = null();
        self.qobject
            .store(n as *mut MessageBuilderQObject, Ordering::SeqCst);
    }

    pub fn body_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.body_changed)(ptr);
        }
    }

    pub fn has_attachments_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.has_attachments_changed)(ptr);
        }
    }

    pub fn is_reply_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.is_reply_changed)(ptr);
        }
    }

    pub fn op_author_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.op_author_changed)(ptr);
        }
    }

    pub fn op_body_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.op_body_changed)(ptr);
        }
    }

    pub fn op_has_attachments_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.op_has_attachments_changed)(ptr);
        }
    }

    pub fn op_id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.op_id_changed)(ptr);
        }
    }

    pub fn op_time_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.op_time_changed)(ptr);
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
pub struct MessageBuilderList {
    pub(super) qobject: *mut MessageBuilderQObject,
    pub(super) layout_about_to_be_changed: fn(*mut MessageBuilderQObject),
    pub(super) layout_changed: fn(*mut MessageBuilderQObject),
    pub(super) begin_reset_model: fn(*mut MessageBuilderQObject),
    pub(super) end_reset_model: fn(*mut MessageBuilderQObject),
    pub(super) end_insert_rows: fn(*mut MessageBuilderQObject),
    pub(super) end_move_rows: fn(*mut MessageBuilderQObject),
    pub(super) end_remove_rows: fn(*mut MessageBuilderQObject),
    pub(super) begin_insert_rows: fn(*mut MessageBuilderQObject, usize, usize),
    pub(super) begin_remove_rows: fn(*mut MessageBuilderQObject, usize, usize),
    pub(super) data_changed: fn(*mut MessageBuilderQObject, usize, usize),
    pub(super) begin_move_rows: fn(*mut MessageBuilderQObject, usize, usize, usize),
}

impl MessageBuilderList {
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

pub trait MessageBuilderTrait {
    fn new(
        emit: MessageBuilderEmitter,
        model: MessageBuilderList,
    ) -> Self;

    fn emit(&mut self) -> &mut MessageBuilderEmitter;

    fn body(&self) -> Option<&str>;

    fn set_body(
        &mut self,
        value: Option<String>,
    );

    fn has_attachments(&self) -> bool;

    fn is_reply(&self) -> bool;

    fn op_author(&self) -> Option<&str>;

    fn op_body(&self) -> Option<&str>;

    fn op_has_attachments(&self) -> Option<bool>;

    fn op_id(&self) -> Option<&[u8]>;

    fn op_time(&self) -> Option<i64>;

    fn add_attachment(
        &mut self,
        path: String,
    ) -> bool;

    fn clear_reply(&mut self) -> ();

    fn finalize(&mut self) -> ();

    fn remove_attachment(
        &mut self,
        path: String,
    ) -> bool;

    fn remove_attachment_by_index(
        &mut self,
        row_index: u64,
    ) -> bool;

    fn remove_last(&mut self) -> ();

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

    fn attachment_path(
        &self,
        index: usize,
    ) -> &str;
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_new(
    ptr_bundle: *mut MessageBuilderPtrBundle
) -> *mut MessageBuilder {
    let d_message_builder = message_builder_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_message_builder))
}

pub unsafe fn message_builder_new_inner(
    ptr_bundle: *mut MessageBuilderPtrBundle
) -> MessageBuilder {
    let ptr_bundle = *ptr_bundle;

    let MessageBuilderPtrBundle {
        message_builder,
        message_builder_body_changed,
        message_builder_has_attachments_changed,
        message_builder_is_reply_changed,
        message_builder_op_author_changed,
        message_builder_op_body_changed,
        message_builder_op_has_attachments_changed,
        message_builder_op_id_changed,
        message_builder_op_time_changed,
        message_builder_new_data_ready,
        message_builder_layout_about_to_be_changed,
        message_builder_layout_changed,
        message_builder_data_changed,
        message_builder_begin_reset_model,
        message_builder_end_reset_model,
        message_builder_begin_insert_rows,
        message_builder_end_insert_rows,
        message_builder_begin_move_rows,
        message_builder_end_move_rows,
        message_builder_begin_remove_rows,
        message_builder_end_remove_rows,
    } = ptr_bundle;
    let message_builder_emit = MessageBuilderEmitter {
        qobject: Arc::new(AtomicPtr::new(message_builder)),
        body_changed: message_builder_body_changed,
        has_attachments_changed: message_builder_has_attachments_changed,
        is_reply_changed: message_builder_is_reply_changed,
        op_author_changed: message_builder_op_author_changed,
        op_body_changed: message_builder_op_body_changed,
        op_has_attachments_changed: message_builder_op_has_attachments_changed,
        op_id_changed: message_builder_op_id_changed,
        op_time_changed: message_builder_op_time_changed,
        new_data_ready: message_builder_new_data_ready,
    };
    let model = MessageBuilderList {
        qobject: message_builder,
        layout_about_to_be_changed: message_builder_layout_about_to_be_changed,
        layout_changed: message_builder_layout_changed,
        data_changed: message_builder_data_changed,
        begin_reset_model: message_builder_begin_reset_model,
        end_reset_model: message_builder_end_reset_model,
        begin_insert_rows: message_builder_begin_insert_rows,
        end_insert_rows: message_builder_end_insert_rows,
        begin_move_rows: message_builder_begin_move_rows,
        end_move_rows: message_builder_end_move_rows,
        begin_remove_rows: message_builder_begin_remove_rows,
        end_remove_rows: message_builder_end_remove_rows,
    };
    let d_message_builder = MessageBuilder::new(message_builder_emit, model);
    d_message_builder
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_free(ptr: *mut MessageBuilder) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_add_attachment(
    ptr: *mut MessageBuilder,
    path_str: *const c_ushort,
    path_len: c_int,
) -> bool {
    let obj = &mut *ptr;
    let mut path = String::new();
    set_string_from_utf16(&mut path, path_str, path_len);
    obj.add_attachment(path)
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_clear_reply(ptr: *mut MessageBuilder) {
    let obj = &mut *ptr;
    obj.clear_reply()
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_finalize(ptr: *mut MessageBuilder) {
    let obj = &mut *ptr;
    obj.finalize()
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_remove_attachment(
    ptr: *mut MessageBuilder,
    path_str: *const c_ushort,
    path_len: c_int,
) -> bool {
    let obj = &mut *ptr;
    let mut path = String::new();
    set_string_from_utf16(&mut path, path_str, path_len);
    obj.remove_attachment(path)
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_remove_attachment_by_index(
    ptr: *mut MessageBuilder,
    row_index: u64,
) -> bool {
    let obj = &mut *ptr;
    obj.remove_attachment_by_index(row_index)
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_remove_last(ptr: *mut MessageBuilder) {
    let obj = &mut *ptr;
    obj.remove_last()
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_body_get(
    ptr: *const MessageBuilder,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.body();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_body_set(
    ptr: *mut MessageBuilder,
    value: *const c_ushort,
    len: c_int,
) {
    let obj = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, value, len);
    obj.set_body(Some(s));
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_body_set_none(ptr: *mut MessageBuilder) {
    let obj = &mut *ptr;
    obj.set_body(None);
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_has_attachments_get(ptr: *const MessageBuilder) -> bool {
    (&*ptr).has_attachments()
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_is_reply_get(ptr: *const MessageBuilder) -> bool {
    (&*ptr).is_reply()
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_op_author_get(
    ptr: *const MessageBuilder,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.op_author();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_op_body_get(
    ptr: *const MessageBuilder,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.op_body();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_op_has_attachments_get(
    ptr: *const MessageBuilder
) -> COption<bool> {
    match (&*ptr).op_has_attachments() {
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
pub unsafe extern "C" fn message_builder_op_id_get(
    ptr: *const MessageBuilder,
    prop: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.op_id();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_op_time_get(ptr: *const MessageBuilder) -> COption<i64> {
    match (&*ptr).op_time() {
        Some(value) => COption {
            data: value,
            some: true,
        },
        None => COption {
            data: i64::default(),
            some: false,
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_row_count(ptr: *const MessageBuilder) -> c_int {
    to_c_int((&*ptr).row_count())
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_insert_rows(
    ptr: *mut MessageBuilder,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).insert_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_remove_rows(
    ptr: *mut MessageBuilder,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).remove_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_can_fetch_more(ptr: *const MessageBuilder) -> bool {
    (&*ptr).can_fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_fetch_more(ptr: *mut MessageBuilder) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_sort(
    ptr: *mut MessageBuilder,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn message_builder_data_attachment_path(
    ptr: *const MessageBuilder,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.attachment_path(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MessageBuilderPtrBundle {
    message_builder: *mut MessageBuilderQObject,
    message_builder_body_changed: fn(*mut MessageBuilderQObject),
    message_builder_has_attachments_changed: fn(*mut MessageBuilderQObject),
    message_builder_is_reply_changed: fn(*mut MessageBuilderQObject),
    message_builder_op_author_changed: fn(*mut MessageBuilderQObject),
    message_builder_op_body_changed: fn(*mut MessageBuilderQObject),
    message_builder_op_has_attachments_changed: fn(*mut MessageBuilderQObject),
    message_builder_op_id_changed: fn(*mut MessageBuilderQObject),
    message_builder_op_time_changed: fn(*mut MessageBuilderQObject),
    message_builder_new_data_ready: fn(*mut MessageBuilderQObject),
    message_builder_layout_about_to_be_changed: fn(*mut MessageBuilderQObject),
    message_builder_layout_changed: fn(*mut MessageBuilderQObject),
    message_builder_data_changed: fn(*mut MessageBuilderQObject, usize, usize),
    message_builder_begin_reset_model: fn(*mut MessageBuilderQObject),
    message_builder_end_reset_model: fn(*mut MessageBuilderQObject),
    message_builder_begin_insert_rows: fn(*mut MessageBuilderQObject, usize, usize),
    message_builder_end_insert_rows: fn(*mut MessageBuilderQObject),
    message_builder_begin_move_rows: fn(*mut MessageBuilderQObject, usize, usize, usize),
    message_builder_end_move_rows: fn(*mut MessageBuilderQObject),
    message_builder_begin_remove_rows: fn(*mut MessageBuilderQObject, usize, usize),
    message_builder_end_remove_rows: fn(*mut MessageBuilderQObject),
}
