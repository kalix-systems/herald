use super::*;

pub struct MessagesQObject;

pub struct MessagesEmitter {
    pub(super) qobject: Arc<AtomicPtr<MessagesQObject>>,
    pub(super) is_empty_changed: fn(*mut MessagesQObject),
    pub(super) last_author_changed: fn(*mut MessagesQObject),
    pub(super) last_body_changed: fn(*mut MessagesQObject),
    pub(super) last_status_changed: fn(*mut MessagesQObject),
    pub(super) last_time_changed: fn(*mut MessagesQObject),
    pub(super) search_active_changed: fn(*mut MessagesQObject),
    pub(super) search_index_changed: fn(*mut MessagesQObject),
    pub(super) search_num_matches_changed: fn(*mut MessagesQObject),
    pub(super) search_pattern_changed: fn(*mut MessagesQObject),
    pub(super) search_regex_changed: fn(*mut MessagesQObject),
    pub(super) new_data_ready: fn(*mut MessagesQObject),
}

impl MessagesEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> MessagesEmitter {
        MessagesEmitter {
            qobject: self.qobject.clone(),
            is_empty_changed: self.is_empty_changed,
            last_author_changed: self.last_author_changed,
            last_body_changed: self.last_body_changed,
            last_status_changed: self.last_status_changed,
            last_time_changed: self.last_time_changed,
            search_active_changed: self.search_active_changed,
            search_index_changed: self.search_index_changed,
            search_num_matches_changed: self.search_num_matches_changed,
            search_pattern_changed: self.search_pattern_changed,
            search_regex_changed: self.search_regex_changed,
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const MessagesQObject = null();
        self.qobject
            .store(n as *mut MessagesQObject, Ordering::SeqCst);
    }

    pub fn is_empty_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.is_empty_changed)(ptr);
        }
    }

    pub fn last_author_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.last_author_changed)(ptr);
        }
    }

    pub fn last_body_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.last_body_changed)(ptr);
        }
    }

    pub fn last_status_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.last_status_changed)(ptr);
        }
    }

    pub fn last_time_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.last_time_changed)(ptr);
        }
    }

    pub fn search_active_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.search_active_changed)(ptr);
        }
    }

    pub fn search_index_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.search_index_changed)(ptr);
        }
    }

    pub fn search_num_matches_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.search_num_matches_changed)(ptr);
        }
    }

    pub fn search_pattern_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.search_pattern_changed)(ptr);
        }
    }

    pub fn search_regex_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.search_regex_changed)(ptr);
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
pub struct MessagesList {
    pub(super) qobject: *mut MessagesQObject,
    pub(super) layout_about_to_be_changed: fn(*mut MessagesQObject),
    pub(super) layout_changed: fn(*mut MessagesQObject),
    pub(super) begin_reset_model: fn(*mut MessagesQObject),
    pub(super) end_reset_model: fn(*mut MessagesQObject),
    pub(super) end_insert_rows: fn(*mut MessagesQObject),
    pub(super) end_move_rows: fn(*mut MessagesQObject),
    pub(super) end_remove_rows: fn(*mut MessagesQObject),
    pub(super) begin_insert_rows: fn(*mut MessagesQObject, usize, usize),
    pub(super) begin_remove_rows: fn(*mut MessagesQObject, usize, usize),
    pub(super) data_changed: fn(*mut MessagesQObject, usize, usize),
    pub(super) begin_move_rows: fn(*mut MessagesQObject, usize, usize, usize),
}

impl MessagesList {
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

pub trait MessagesTrait {
    fn new(
        emit: MessagesEmitter,
        model: MessagesList,
        builder: MessageBuilder,
    ) -> Self;

    fn emit(&mut self) -> &mut MessagesEmitter;

    fn builder(&self) -> &MessageBuilder;

    fn builder_mut(&mut self) -> &mut MessageBuilder;

    fn is_empty(&self) -> bool;

    fn last_author(&self) -> Option<&str>;

    fn last_body(&self) -> Option<&str>;

    fn last_status(&self) -> Option<u32>;

    fn last_time(&self) -> Option<i64>;

    fn search_active(&self) -> bool;

    fn set_search_active(
        &mut self,
        value: bool,
    );

    fn search_index(&self) -> u64;

    fn search_num_matches(&self) -> u64;

    fn search_pattern(&self) -> &str;

    fn set_search_pattern(
        &mut self,
        value: String,
    );

    fn search_regex(&self) -> bool;

    fn set_search_regex(
        &mut self,
        value: bool,
    );

    fn clear_conversation_history(&mut self) -> bool;

    fn clear_search(&mut self) -> ();

    fn delete_message(
        &mut self,
        row_index: u64,
    ) -> bool;

    fn index_by_id(
        &self,
        msg_id: &[u8],
    ) -> i64;

    fn next_search_match(&mut self) -> i64;

    fn prev_search_match(&mut self) -> i64;

    fn save_all_attachments(
        &self,
        index: u64,
        dest: String,
    ) -> bool;

    fn set_elision_char_count(
        &mut self,
        char_count: u16,
    ) -> ();

    fn set_elision_chars_per_line(
        &mut self,
        chars_per_line: u8,
    ) -> ();

    fn set_elision_line_count(
        &mut self,
        line_count: u8,
    ) -> ();

    fn set_search_hint(
        &mut self,
        scrollbar_position: f32,
        scrollbar_height: f32,
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

    fn author(
        &self,
        index: usize,
    ) -> Option<String>;

    fn author_color(
        &self,
        index: usize,
    ) -> Option<u32>;

    fn author_name(
        &self,
        index: usize,
    ) -> Option<String>;

    fn author_profile_picture(
        &self,
        index: usize,
    ) -> String;

    fn body(
        &self,
        index: usize,
    ) -> String;

    fn doc_attachments(
        &self,
        index: usize,
    ) -> String;

    fn expiration_time(
        &self,
        index: usize,
    ) -> Option<i64>;

    fn full_body(
        &self,
        index: usize,
    ) -> String;

    fn full_media_attachments(
        &self,
        index: usize,
    ) -> String;

    fn insertion_time(
        &self,
        index: usize,
    ) -> Option<i64>;

    fn is_head(
        &self,
        index: usize,
    ) -> Option<bool>;

    fn is_tail(
        &self,
        index: usize,
    ) -> Option<bool>;

    fn match_status(
        &self,
        index: usize,
    ) -> Option<u8>;

    fn media_attachments(
        &self,
        index: usize,
    ) -> String;

    fn msg_id(
        &self,
        index: usize,
    ) -> Option<&[u8]>;

    fn op_author(
        &self,
        index: usize,
    ) -> Option<String>;

    fn op_body(
        &self,
        index: usize,
    ) -> String;

    fn op_color(
        &self,
        index: usize,
    ) -> Option<u32>;

    fn op_doc_attachments(
        &self,
        index: usize,
    ) -> String;

    fn op_expiration_time(
        &self,
        index: usize,
    ) -> Option<i64>;

    fn op_insertion_time(
        &self,
        index: usize,
    ) -> Option<i64>;

    fn op_media_attachments(
        &self,
        index: usize,
    ) -> String;

    fn op_msg_id(
        &self,
        index: usize,
    ) -> Option<Vec<u8>>;

    fn op_name(
        &self,
        index: usize,
    ) -> Option<String>;

    fn receipt_status(
        &self,
        index: usize,
    ) -> Option<u32>;

    fn reply_type(
        &self,
        index: usize,
    ) -> Option<u8>;

    fn server_time(
        &self,
        index: usize,
    ) -> Option<i64>;
}

#[no_mangle]
pub unsafe extern "C" fn messages_new(ptr_bundle: *mut MessagesPtrBundle) -> *mut Messages {
    let d_messages = messages_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_messages))
}

pub unsafe fn messages_new_inner(ptr_bundle: *mut MessagesPtrBundle) -> Messages {
    let ptr_bundle = *ptr_bundle;

    let MessagesPtrBundle {
        messages,
        builder,
        builder_body_changed,
        document_attachments,
        document_attachments_new_data_ready,
        document_attachments_layout_about_to_be_changed,
        document_attachments_layout_changed,
        document_attachments_data_changed,
        document_attachments_begin_reset_model,
        document_attachments_end_reset_model,
        document_attachments_begin_insert_rows,
        document_attachments_end_insert_rows,
        document_attachments_begin_move_rows,
        document_attachments_end_move_rows,
        document_attachments_begin_remove_rows,
        document_attachments_end_remove_rows,
        builder_has_doc_attachment_changed,
        builder_has_media_attachment_changed,
        builder_is_reply_changed,
        media_attachments,
        media_attachments_new_data_ready,
        media_attachments_layout_about_to_be_changed,
        media_attachments_layout_changed,
        media_attachments_data_changed,
        media_attachments_begin_reset_model,
        media_attachments_end_reset_model,
        media_attachments_begin_insert_rows,
        media_attachments_end_insert_rows,
        media_attachments_begin_move_rows,
        media_attachments_end_move_rows,
        media_attachments_begin_remove_rows,
        media_attachments_end_remove_rows,
        builder_op_author_changed,
        builder_op_body_changed,
        builder_op_doc_attachments_changed,
        builder_op_expiration_time_changed,
        builder_op_id_changed,
        builder_op_media_attachments_changed,
        builder_op_time_changed,
        builder_new_data_ready,
        builder_layout_about_to_be_changed,
        builder_layout_changed,
        builder_data_changed,
        builder_begin_reset_model,
        builder_end_reset_model,
        builder_begin_insert_rows,
        builder_end_insert_rows,
        builder_begin_move_rows,
        builder_end_move_rows,
        builder_begin_remove_rows,
        builder_end_remove_rows,
        messages_is_empty_changed,
        messages_last_author_changed,
        messages_last_body_changed,
        messages_last_status_changed,
        messages_last_time_changed,
        messages_search_active_changed,
        messages_search_index_changed,
        messages_search_num_matches_changed,
        messages_search_pattern_changed,
        messages_search_regex_changed,
        messages_new_data_ready,
        messages_layout_about_to_be_changed,
        messages_layout_changed,
        messages_data_changed,
        messages_begin_reset_model,
        messages_end_reset_model,
        messages_begin_insert_rows,
        messages_end_insert_rows,
        messages_begin_move_rows,
        messages_end_move_rows,
        messages_begin_remove_rows,
        messages_end_remove_rows,
    } = ptr_bundle;
    let document_attachments_emit = DocumentAttachmentsEmitter {
        qobject: Arc::new(AtomicPtr::new(document_attachments)),
        new_data_ready: document_attachments_new_data_ready,
    };
    let model = DocumentAttachmentsList {
        qobject: document_attachments,
        layout_about_to_be_changed: document_attachments_layout_about_to_be_changed,
        layout_changed: document_attachments_layout_changed,
        data_changed: document_attachments_data_changed,
        begin_reset_model: document_attachments_begin_reset_model,
        end_reset_model: document_attachments_end_reset_model,
        begin_insert_rows: document_attachments_begin_insert_rows,
        end_insert_rows: document_attachments_end_insert_rows,
        begin_move_rows: document_attachments_begin_move_rows,
        end_move_rows: document_attachments_end_move_rows,
        begin_remove_rows: document_attachments_begin_remove_rows,
        end_remove_rows: document_attachments_end_remove_rows,
    };
    let d_document_attachments = DocumentAttachments::new(document_attachments_emit, model);
    let media_attachments_emit = MediaAttachmentsEmitter {
        qobject: Arc::new(AtomicPtr::new(media_attachments)),
        new_data_ready: media_attachments_new_data_ready,
    };
    let model = MediaAttachmentsList {
        qobject: media_attachments,
        layout_about_to_be_changed: media_attachments_layout_about_to_be_changed,
        layout_changed: media_attachments_layout_changed,
        data_changed: media_attachments_data_changed,
        begin_reset_model: media_attachments_begin_reset_model,
        end_reset_model: media_attachments_end_reset_model,
        begin_insert_rows: media_attachments_begin_insert_rows,
        end_insert_rows: media_attachments_end_insert_rows,
        begin_move_rows: media_attachments_begin_move_rows,
        end_move_rows: media_attachments_end_move_rows,
        begin_remove_rows: media_attachments_begin_remove_rows,
        end_remove_rows: media_attachments_end_remove_rows,
    };
    let d_media_attachments = MediaAttachments::new(media_attachments_emit, model);
    let builder_emit = MessageBuilderEmitter {
        qobject: Arc::new(AtomicPtr::new(builder)),
        body_changed: builder_body_changed,
        has_doc_attachment_changed: builder_has_doc_attachment_changed,
        has_media_attachment_changed: builder_has_media_attachment_changed,
        is_reply_changed: builder_is_reply_changed,
        op_author_changed: builder_op_author_changed,
        op_body_changed: builder_op_body_changed,
        op_doc_attachments_changed: builder_op_doc_attachments_changed,
        op_expiration_time_changed: builder_op_expiration_time_changed,
        op_id_changed: builder_op_id_changed,
        op_media_attachments_changed: builder_op_media_attachments_changed,
        op_time_changed: builder_op_time_changed,
        new_data_ready: builder_new_data_ready,
    };
    let model = MessageBuilderList {
        qobject: builder,
        layout_about_to_be_changed: builder_layout_about_to_be_changed,
        layout_changed: builder_layout_changed,
        data_changed: builder_data_changed,
        begin_reset_model: builder_begin_reset_model,
        end_reset_model: builder_end_reset_model,
        begin_insert_rows: builder_begin_insert_rows,
        end_insert_rows: builder_end_insert_rows,
        begin_move_rows: builder_begin_move_rows,
        end_move_rows: builder_end_move_rows,
        begin_remove_rows: builder_begin_remove_rows,
        end_remove_rows: builder_end_remove_rows,
    };
    let d_builder = MessageBuilder::new(
        builder_emit,
        model,
        d_document_attachments,
        d_media_attachments,
    );
    let messages_emit = MessagesEmitter {
        qobject: Arc::new(AtomicPtr::new(messages)),
        is_empty_changed: messages_is_empty_changed,
        last_author_changed: messages_last_author_changed,
        last_body_changed: messages_last_body_changed,
        last_status_changed: messages_last_status_changed,
        last_time_changed: messages_last_time_changed,
        search_active_changed: messages_search_active_changed,
        search_index_changed: messages_search_index_changed,
        search_num_matches_changed: messages_search_num_matches_changed,
        search_pattern_changed: messages_search_pattern_changed,
        search_regex_changed: messages_search_regex_changed,
        new_data_ready: messages_new_data_ready,
    };
    let model = MessagesList {
        qobject: messages,
        layout_about_to_be_changed: messages_layout_about_to_be_changed,
        layout_changed: messages_layout_changed,
        data_changed: messages_data_changed,
        begin_reset_model: messages_begin_reset_model,
        end_reset_model: messages_end_reset_model,
        begin_insert_rows: messages_begin_insert_rows,
        end_insert_rows: messages_end_insert_rows,
        begin_move_rows: messages_begin_move_rows,
        end_move_rows: messages_end_move_rows,
        begin_remove_rows: messages_begin_remove_rows,
        end_remove_rows: messages_end_remove_rows,
    };
    let d_messages = Messages::new(messages_emit, model, d_builder);
    d_messages
}

#[no_mangle]
pub unsafe extern "C" fn messages_free(ptr: *mut Messages) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn messages_clear_conversation_history(ptr: *mut Messages) -> bool {
    let obj = &mut *ptr;
    obj.clear_conversation_history()
}

#[no_mangle]
pub unsafe extern "C" fn messages_clear_search(ptr: *mut Messages) {
    let obj = &mut *ptr;
    obj.clear_search()
}

#[no_mangle]
pub unsafe extern "C" fn messages_delete_message(
    ptr: *mut Messages,
    row_index: u64,
) -> bool {
    let obj = &mut *ptr;
    obj.delete_message(row_index)
}

#[no_mangle]
pub unsafe extern "C" fn messages_index_by_id(
    ptr: *const Messages,
    msg_id_str: *const c_char,
    msg_id_len: c_int,
) -> i64 {
    let obj = &*ptr;
    let msg_id = { qba_slice!(msg_id_str, msg_id_len) };
    obj.index_by_id(msg_id)
}

#[no_mangle]
pub unsafe extern "C" fn messages_next_search_match(ptr: *mut Messages) -> i64 {
    let obj = &mut *ptr;
    obj.next_search_match()
}

#[no_mangle]
pub unsafe extern "C" fn messages_prev_search_match(ptr: *mut Messages) -> i64 {
    let obj = &mut *ptr;
    obj.prev_search_match()
}

#[no_mangle]
pub unsafe extern "C" fn messages_save_all_attachments(
    ptr: *const Messages,
    index: u64,
    dest_str: *const c_ushort,
    dest_len: c_int,
) -> bool {
    let obj = &*ptr;
    let mut dest = String::new();
    set_string_from_utf16(&mut dest, dest_str, dest_len);
    obj.save_all_attachments(index, dest)
}

#[no_mangle]
pub unsafe extern "C" fn messages_set_elision_char_count(
    ptr: *mut Messages,
    char_count: u16,
) {
    let obj = &mut *ptr;
    obj.set_elision_char_count(char_count)
}

#[no_mangle]
pub unsafe extern "C" fn messages_set_elision_chars_per_line(
    ptr: *mut Messages,
    chars_per_line: u8,
) {
    let obj = &mut *ptr;
    obj.set_elision_chars_per_line(chars_per_line)
}

#[no_mangle]
pub unsafe extern "C" fn messages_set_elision_line_count(
    ptr: *mut Messages,
    line_count: u8,
) {
    let obj = &mut *ptr;
    obj.set_elision_line_count(line_count)
}

#[no_mangle]
pub unsafe extern "C" fn messages_set_search_hint(
    ptr: *mut Messages,
    scrollbar_position: f32,
    scrollbar_height: f32,
) {
    let obj = &mut *ptr;
    obj.set_search_hint(scrollbar_position, scrollbar_height)
}

#[no_mangle]
pub unsafe extern "C" fn messages_builder_get(ptr: *mut Messages) -> *mut MessageBuilder {
    (&mut *ptr).builder_mut()
}

#[no_mangle]
pub unsafe extern "C" fn messages_is_empty_get(ptr: *const Messages) -> bool {
    (&*ptr).is_empty()
}

#[no_mangle]
pub unsafe extern "C" fn messages_last_author_get(
    ptr: *const Messages,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.last_author();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn messages_last_body_get(
    ptr: *const Messages,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.last_body();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn messages_last_status_get(ptr: *const Messages) -> COption<u32> {
    match (&*ptr).last_status() {
        Some(value) => COption {
            data: value,
            some: true,
        },
        None => COption {
            data: u32::default(),
            some: false,
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn messages_last_time_get(ptr: *const Messages) -> COption<i64> {
    match (&*ptr).last_time() {
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
pub unsafe extern "C" fn messages_search_active_get(ptr: *const Messages) -> bool {
    (&*ptr).search_active()
}

#[no_mangle]
pub unsafe extern "C" fn messages_search_active_set(
    ptr: *mut Messages,
    value: bool,
) {
    (&mut *ptr).set_search_active(value)
}

#[no_mangle]
pub unsafe extern "C" fn messages_search_index_get(ptr: *const Messages) -> u64 {
    (&*ptr).search_index()
}

#[no_mangle]
pub unsafe extern "C" fn messages_search_num_matches_get(ptr: *const Messages) -> u64 {
    (&*ptr).search_num_matches()
}

#[no_mangle]
pub unsafe extern "C" fn messages_search_pattern_get(
    ptr: *const Messages,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.search_pattern();
    let str_: *const c_char = value.as_ptr() as *const c_char;
    set(prop, str_, to_c_int(value.len()));
}

#[no_mangle]
pub unsafe extern "C" fn messages_search_pattern_set(
    ptr: *mut Messages,
    value: *const c_ushort,
    len: c_int,
) {
    let obj = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, value, len);
    obj.set_search_pattern(s);
}

#[no_mangle]
pub unsafe extern "C" fn messages_search_regex_get(ptr: *const Messages) -> bool {
    (&*ptr).search_regex()
}

#[no_mangle]
pub unsafe extern "C" fn messages_search_regex_set(
    ptr: *mut Messages,
    value: bool,
) {
    (&mut *ptr).set_search_regex(value)
}

#[no_mangle]
pub unsafe extern "C" fn messages_row_count(ptr: *const Messages) -> c_int {
    to_c_int((&*ptr).row_count())
}

#[no_mangle]
pub unsafe extern "C" fn messages_insert_rows(
    ptr: *mut Messages,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).insert_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn messages_remove_rows(
    ptr: *mut Messages,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).remove_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn messages_can_fetch_more(ptr: *const Messages) -> bool {
    (&*ptr).can_fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn messages_fetch_more(ptr: *mut Messages) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn messages_sort(
    ptr: *mut Messages,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_author(
    ptr: *const Messages,
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
pub unsafe extern "C" fn messages_data_author_color(
    ptr: *const Messages,
    row: c_int,
) -> COption<u32> {
    let obj = &*ptr;
    obj.author_color(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_author_name(
    ptr: *const Messages,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.author_name(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_author_profile_picture(
    ptr: *const Messages,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.author_profile_picture(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_body(
    ptr: *const Messages,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.body(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_doc_attachments(
    ptr: *const Messages,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.doc_attachments(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_expiration_time(
    ptr: *const Messages,
    row: c_int,
) -> COption<i64> {
    let obj = &*ptr;
    obj.expiration_time(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_full_body(
    ptr: *const Messages,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.full_body(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_full_media_attachments(
    ptr: *const Messages,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.full_media_attachments(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_insertion_time(
    ptr: *const Messages,
    row: c_int,
) -> COption<i64> {
    let obj = &*ptr;
    obj.insertion_time(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_is_head(
    ptr: *const Messages,
    row: c_int,
) -> COption<bool> {
    let obj = &*ptr;
    obj.is_head(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_is_tail(
    ptr: *const Messages,
    row: c_int,
) -> COption<bool> {
    let obj = &*ptr;
    obj.is_tail(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_match_status(
    ptr: *const Messages,
    row: c_int,
) -> COption<u8> {
    let obj = &*ptr;
    obj.match_status(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_media_attachments(
    ptr: *const Messages,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.media_attachments(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_msg_id(
    ptr: *const Messages,
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
pub unsafe extern "C" fn messages_data_op_author(
    ptr: *const Messages,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.op_author(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_op_body(
    ptr: *const Messages,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.op_body(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_op_color(
    ptr: *const Messages,
    row: c_int,
) -> COption<u32> {
    let obj = &*ptr;
    obj.op_color(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_op_doc_attachments(
    ptr: *const Messages,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.op_doc_attachments(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_op_expiration_time(
    ptr: *const Messages,
    row: c_int,
) -> COption<i64> {
    let obj = &*ptr;
    obj.op_expiration_time(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_op_insertion_time(
    ptr: *const Messages,
    row: c_int,
) -> COption<i64> {
    let obj = &*ptr;
    obj.op_insertion_time(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_op_media_attachments(
    ptr: *const Messages,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.op_media_attachments(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_op_msg_id(
    ptr: *const Messages,
    row: c_int,
    d: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.op_msg_id(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_op_name(
    ptr: *const Messages,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.op_name(to_usize(row).unwrap_or(0));
    if let Some(data) = data {
        let str_: *const c_char = data.as_ptr() as (*const c_char);
        set(d, str_, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_receipt_status(
    ptr: *const Messages,
    row: c_int,
) -> COption<u32> {
    let obj = &*ptr;
    obj.receipt_status(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_reply_type(
    ptr: *const Messages,
    row: c_int,
) -> COption<u8> {
    let obj = &*ptr;
    obj.reply_type(to_usize(row).unwrap_or(0)).into()
}

#[no_mangle]
pub unsafe extern "C" fn messages_data_server_time(
    ptr: *const Messages,
    row: c_int,
) -> COption<i64> {
    let obj = &*ptr;
    obj.server_time(to_usize(row).unwrap_or(0)).into()
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MessagesPtrBundle {
    messages: *mut MessagesQObject,
    builder: *mut MessageBuilderQObject,
    builder_body_changed: fn(*mut MessageBuilderQObject),
    document_attachments: *mut DocumentAttachmentsQObject,
    document_attachments_new_data_ready: fn(*mut DocumentAttachmentsQObject),
    document_attachments_layout_about_to_be_changed: fn(*mut DocumentAttachmentsQObject),
    document_attachments_layout_changed: fn(*mut DocumentAttachmentsQObject),
    document_attachments_data_changed: fn(*mut DocumentAttachmentsQObject, usize, usize),
    document_attachments_begin_reset_model: fn(*mut DocumentAttachmentsQObject),
    document_attachments_end_reset_model: fn(*mut DocumentAttachmentsQObject),
    document_attachments_begin_insert_rows: fn(*mut DocumentAttachmentsQObject, usize, usize),
    document_attachments_end_insert_rows: fn(*mut DocumentAttachmentsQObject),
    document_attachments_begin_move_rows: fn(*mut DocumentAttachmentsQObject, usize, usize, usize),
    document_attachments_end_move_rows: fn(*mut DocumentAttachmentsQObject),
    document_attachments_begin_remove_rows: fn(*mut DocumentAttachmentsQObject, usize, usize),
    document_attachments_end_remove_rows: fn(*mut DocumentAttachmentsQObject),
    builder_has_doc_attachment_changed: fn(*mut MessageBuilderQObject),
    builder_has_media_attachment_changed: fn(*mut MessageBuilderQObject),
    builder_is_reply_changed: fn(*mut MessageBuilderQObject),
    media_attachments: *mut MediaAttachmentsQObject,
    media_attachments_new_data_ready: fn(*mut MediaAttachmentsQObject),
    media_attachments_layout_about_to_be_changed: fn(*mut MediaAttachmentsQObject),
    media_attachments_layout_changed: fn(*mut MediaAttachmentsQObject),
    media_attachments_data_changed: fn(*mut MediaAttachmentsQObject, usize, usize),
    media_attachments_begin_reset_model: fn(*mut MediaAttachmentsQObject),
    media_attachments_end_reset_model: fn(*mut MediaAttachmentsQObject),
    media_attachments_begin_insert_rows: fn(*mut MediaAttachmentsQObject, usize, usize),
    media_attachments_end_insert_rows: fn(*mut MediaAttachmentsQObject),
    media_attachments_begin_move_rows: fn(*mut MediaAttachmentsQObject, usize, usize, usize),
    media_attachments_end_move_rows: fn(*mut MediaAttachmentsQObject),
    media_attachments_begin_remove_rows: fn(*mut MediaAttachmentsQObject, usize, usize),
    media_attachments_end_remove_rows: fn(*mut MediaAttachmentsQObject),
    builder_op_author_changed: fn(*mut MessageBuilderQObject),
    builder_op_body_changed: fn(*mut MessageBuilderQObject),
    builder_op_doc_attachments_changed: fn(*mut MessageBuilderQObject),
    builder_op_expiration_time_changed: fn(*mut MessageBuilderQObject),
    builder_op_id_changed: fn(*mut MessageBuilderQObject),
    builder_op_media_attachments_changed: fn(*mut MessageBuilderQObject),
    builder_op_time_changed: fn(*mut MessageBuilderQObject),
    builder_new_data_ready: fn(*mut MessageBuilderQObject),
    builder_layout_about_to_be_changed: fn(*mut MessageBuilderQObject),
    builder_layout_changed: fn(*mut MessageBuilderQObject),
    builder_data_changed: fn(*mut MessageBuilderQObject, usize, usize),
    builder_begin_reset_model: fn(*mut MessageBuilderQObject),
    builder_end_reset_model: fn(*mut MessageBuilderQObject),
    builder_begin_insert_rows: fn(*mut MessageBuilderQObject, usize, usize),
    builder_end_insert_rows: fn(*mut MessageBuilderQObject),
    builder_begin_move_rows: fn(*mut MessageBuilderQObject, usize, usize, usize),
    builder_end_move_rows: fn(*mut MessageBuilderQObject),
    builder_begin_remove_rows: fn(*mut MessageBuilderQObject, usize, usize),
    builder_end_remove_rows: fn(*mut MessageBuilderQObject),
    messages_is_empty_changed: fn(*mut MessagesQObject),
    messages_last_author_changed: fn(*mut MessagesQObject),
    messages_last_body_changed: fn(*mut MessagesQObject),
    messages_last_status_changed: fn(*mut MessagesQObject),
    messages_last_time_changed: fn(*mut MessagesQObject),
    messages_search_active_changed: fn(*mut MessagesQObject),
    messages_search_index_changed: fn(*mut MessagesQObject),
    messages_search_num_matches_changed: fn(*mut MessagesQObject),
    messages_search_pattern_changed: fn(*mut MessagesQObject),
    messages_search_regex_changed: fn(*mut MessagesQObject),
    messages_new_data_ready: fn(*mut MessagesQObject),
    messages_layout_about_to_be_changed: fn(*mut MessagesQObject),
    messages_layout_changed: fn(*mut MessagesQObject),
    messages_data_changed: fn(*mut MessagesQObject, usize, usize),
    messages_begin_reset_model: fn(*mut MessagesQObject),
    messages_end_reset_model: fn(*mut MessagesQObject),
    messages_begin_insert_rows: fn(*mut MessagesQObject, usize, usize),
    messages_end_insert_rows: fn(*mut MessagesQObject),
    messages_begin_move_rows: fn(*mut MessagesQObject, usize, usize, usize),
    messages_end_move_rows: fn(*mut MessagesQObject),
    messages_begin_remove_rows: fn(*mut MessagesQObject, usize, usize),
    messages_end_remove_rows: fn(*mut MessagesQObject),
}
