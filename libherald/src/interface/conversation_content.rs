use super::*;

pub struct ConversationContentQObject;

pub struct ConversationContentEmitter {
    pub(super) qobject: Arc<AtomicPtr<ConversationContentQObject>>,
    pub(super) conversation_id_changed: fn(*mut ConversationContentQObject),
    pub(super) new_data_ready: fn(*mut ConversationContentQObject),
}

impl ConversationContentEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> ConversationContentEmitter {
        ConversationContentEmitter {
            qobject: self.qobject.clone(),
            conversation_id_changed: self.conversation_id_changed,
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const ConversationContentQObject = null();
        self.qobject
            .store(n as *mut ConversationContentQObject, Ordering::SeqCst);
    }

    pub fn conversation_id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.conversation_id_changed)(ptr);
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
pub struct ConversationContentList {
    pub(super) qobject: *mut ConversationContentQObject,
    pub(super) layout_about_to_be_changed: fn(*mut ConversationContentQObject),
    pub(super) layout_changed: fn(*mut ConversationContentQObject),
    pub(super) begin_reset_model: fn(*mut ConversationContentQObject),
    pub(super) end_reset_model: fn(*mut ConversationContentQObject),
    pub(super) end_insert_rows: fn(*mut ConversationContentQObject),
    pub(super) end_move_rows: fn(*mut ConversationContentQObject),
    pub(super) end_remove_rows: fn(*mut ConversationContentQObject),
    pub(super) begin_insert_rows: fn(*mut ConversationContentQObject, usize, usize),
    pub(super) begin_remove_rows: fn(*mut ConversationContentQObject, usize, usize),
    pub(super) data_changed: fn(*mut ConversationContentQObject, usize, usize),
    pub(super) begin_move_rows: fn(*mut ConversationContentQObject, usize, usize, usize),
}

impl ConversationContentList {
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

pub trait ConversationContentTrait {
    fn new(
        emit: ConversationContentEmitter,
        model: ConversationContentList,
        members: Members,
        messages: Messages,
    ) -> Self;

    fn emit(&mut self) -> &mut ConversationContentEmitter;

    fn conversation_id(&self) -> Option<&[u8]>;

    fn set_conversation_id(
        &mut self,
        value: Option<&[u8]>,
    );

    fn members(&self) -> &Members;

    fn members_mut(&mut self) -> &mut Members;

    fn messages(&self) -> &Messages;

    fn messages_mut(&mut self) -> &mut Messages;

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
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_new(
    ptr_bundle: *mut ConversationContentPtrBundle
) -> *mut ConversationContent {
    let d_conversation_content = conversation_content_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_conversation_content))
}

pub unsafe fn conversation_content_new_inner(
    ptr_bundle: *mut ConversationContentPtrBundle
) -> ConversationContent {
    let ptr_bundle = *ptr_bundle;

    let ConversationContentPtrBundle {
        conversation_content,
        conversation_content_conversation_id_changed,
        members,
        members_filter_changed,
        members_filter_regex_changed,
        members_new_data_ready,
        members_layout_about_to_be_changed,
        members_layout_changed,
        members_data_changed,
        members_begin_reset_model,
        members_end_reset_model,
        members_begin_insert_rows,
        members_end_insert_rows,
        members_begin_move_rows,
        members_end_move_rows,
        members_begin_remove_rows,
        members_end_remove_rows,
        messages,
        builder,
        builder_body_changed,
        builder_is_media_message_changed,
        builder_is_reply_changed,
        builder_op_author_changed,
        builder_op_body_changed,
        builder_op_has_attachments_changed,
        builder_op_id_changed,
        builder_op_time_changed,
        builder_parse_markdown_changed,
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
        messages_builder_op_msg_id_changed,
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
        conversation_content_new_data_ready,
        conversation_content_layout_about_to_be_changed,
        conversation_content_layout_changed,
        conversation_content_data_changed,
        conversation_content_begin_reset_model,
        conversation_content_end_reset_model,
        conversation_content_begin_insert_rows,
        conversation_content_end_insert_rows,
        conversation_content_begin_move_rows,
        conversation_content_end_move_rows,
        conversation_content_begin_remove_rows,
        conversation_content_end_remove_rows,
    } = ptr_bundle;
    let members_emit = MembersEmitter {
        qobject: Arc::new(AtomicPtr::new(members)),
        filter_changed: members_filter_changed,
        filter_regex_changed: members_filter_regex_changed,
        new_data_ready: members_new_data_ready,
    };
    let model = MembersList {
        qobject: members,
        layout_about_to_be_changed: members_layout_about_to_be_changed,
        layout_changed: members_layout_changed,
        data_changed: members_data_changed,
        begin_reset_model: members_begin_reset_model,
        end_reset_model: members_end_reset_model,
        begin_insert_rows: members_begin_insert_rows,
        end_insert_rows: members_end_insert_rows,
        begin_move_rows: members_begin_move_rows,
        end_move_rows: members_end_move_rows,
        begin_remove_rows: members_begin_remove_rows,
        end_remove_rows: members_end_remove_rows,
    };
    let d_members = Members::new(members_emit, model);
    let builder_emit = MessageBuilderEmitter {
        qobject: Arc::new(AtomicPtr::new(builder)),
        body_changed: builder_body_changed,
        is_media_message_changed: builder_is_media_message_changed,
        is_reply_changed: builder_is_reply_changed,
        op_author_changed: builder_op_author_changed,
        op_body_changed: builder_op_body_changed,
        op_has_attachments_changed: builder_op_has_attachments_changed,
        op_id_changed: builder_op_id_changed,
        op_time_changed: builder_op_time_changed,
        parse_markdown_changed: builder_parse_markdown_changed,
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
    let d_builder = MessageBuilder::new(builder_emit, model);
    let messages_emit = MessagesEmitter {
        qobject: Arc::new(AtomicPtr::new(messages)),
        builder_op_msg_id_changed: messages_builder_op_msg_id_changed,
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
    let conversation_content_emit = ConversationContentEmitter {
        qobject: Arc::new(AtomicPtr::new(conversation_content)),
        conversation_id_changed: conversation_content_conversation_id_changed,
        new_data_ready: conversation_content_new_data_ready,
    };
    let model = ConversationContentList {
        qobject: conversation_content,
        layout_about_to_be_changed: conversation_content_layout_about_to_be_changed,
        layout_changed: conversation_content_layout_changed,
        data_changed: conversation_content_data_changed,
        begin_reset_model: conversation_content_begin_reset_model,
        end_reset_model: conversation_content_end_reset_model,
        begin_insert_rows: conversation_content_begin_insert_rows,
        end_insert_rows: conversation_content_end_insert_rows,
        begin_move_rows: conversation_content_begin_move_rows,
        end_move_rows: conversation_content_end_move_rows,
        begin_remove_rows: conversation_content_begin_remove_rows,
        end_remove_rows: conversation_content_end_remove_rows,
    };
    let d_conversation_content =
        ConversationContent::new(conversation_content_emit, model, d_members, d_messages);
    d_conversation_content
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_free(ptr: *mut ConversationContent) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_conversation_id_get(
    ptr: *const ConversationContent,
    prop: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.conversation_id();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_conversation_id_set(
    ptr: *mut ConversationContent,
    value: *const c_char,
    len: c_int,
) {
    let obj = &mut *ptr;
    let value = qba_slice!(value, len);
    obj.set_conversation_id(Some(value));
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_conversation_id_set_none(
    ptr: *mut ConversationContent
) {
    let obj = &mut *ptr;
    obj.set_conversation_id(None);
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_members_get(
    ptr: *mut ConversationContent
) -> *mut Members {
    (&mut *ptr).members_mut()
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_messages_get(
    ptr: *mut ConversationContent
) -> *mut Messages {
    (&mut *ptr).messages_mut()
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_row_count(ptr: *const ConversationContent) -> c_int {
    to_c_int((&*ptr).row_count())
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_insert_rows(
    ptr: *mut ConversationContent,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).insert_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_remove_rows(
    ptr: *mut ConversationContent,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).remove_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_can_fetch_more(
    ptr: *const ConversationContent
) -> bool {
    (&*ptr).can_fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_fetch_more(ptr: *mut ConversationContent) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_sort(
    ptr: *mut ConversationContent,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ConversationContentPtrBundle {
    conversation_content: *mut ConversationContentQObject,
    conversation_content_conversation_id_changed: fn(*mut ConversationContentQObject),
    members: *mut MembersQObject,
    members_filter_changed: fn(*mut MembersQObject),
    members_filter_regex_changed: fn(*mut MembersQObject),
    members_new_data_ready: fn(*mut MembersQObject),
    members_layout_about_to_be_changed: fn(*mut MembersQObject),
    members_layout_changed: fn(*mut MembersQObject),
    members_data_changed: fn(*mut MembersQObject, usize, usize),
    members_begin_reset_model: fn(*mut MembersQObject),
    members_end_reset_model: fn(*mut MembersQObject),
    members_begin_insert_rows: fn(*mut MembersQObject, usize, usize),
    members_end_insert_rows: fn(*mut MembersQObject),
    members_begin_move_rows: fn(*mut MembersQObject, usize, usize, usize),
    members_end_move_rows: fn(*mut MembersQObject),
    members_begin_remove_rows: fn(*mut MembersQObject, usize, usize),
    members_end_remove_rows: fn(*mut MembersQObject),
    messages: *mut MessagesQObject,
    builder: *mut MessageBuilderQObject,
    builder_body_changed: fn(*mut MessageBuilderQObject),
    builder_is_media_message_changed: fn(*mut MessageBuilderQObject),
    builder_is_reply_changed: fn(*mut MessageBuilderQObject),
    builder_op_author_changed: fn(*mut MessageBuilderQObject),
    builder_op_body_changed: fn(*mut MessageBuilderQObject),
    builder_op_has_attachments_changed: fn(*mut MessageBuilderQObject),
    builder_op_id_changed: fn(*mut MessageBuilderQObject),
    builder_op_time_changed: fn(*mut MessageBuilderQObject),
    builder_parse_markdown_changed: fn(*mut MessageBuilderQObject),
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
    messages_builder_op_msg_id_changed: fn(*mut MessagesQObject),
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
    conversation_content_new_data_ready: fn(*mut ConversationContentQObject),
    conversation_content_layout_about_to_be_changed: fn(*mut ConversationContentQObject),
    conversation_content_layout_changed: fn(*mut ConversationContentQObject),
    conversation_content_data_changed: fn(*mut ConversationContentQObject, usize, usize),
    conversation_content_begin_reset_model: fn(*mut ConversationContentQObject),
    conversation_content_end_reset_model: fn(*mut ConversationContentQObject),
    conversation_content_begin_insert_rows: fn(*mut ConversationContentQObject, usize, usize),
    conversation_content_end_insert_rows: fn(*mut ConversationContentQObject),
    conversation_content_begin_move_rows: fn(*mut ConversationContentQObject, usize, usize, usize),
    conversation_content_end_move_rows: fn(*mut ConversationContentQObject),
    conversation_content_begin_remove_rows: fn(*mut ConversationContentQObject, usize, usize),
    conversation_content_end_remove_rows: fn(*mut ConversationContentQObject),
}
