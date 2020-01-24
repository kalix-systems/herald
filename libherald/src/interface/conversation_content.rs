use super::*;

pub struct ConversationContentQObject;

pub struct ConversationContentEmitter {
    pub(super) qobject: Arc<AtomicPtr<ConversationContentQObject>>,
    pub(super) conversation_color_changed: fn(*mut ConversationContentQObject),
    pub(super) conversation_id_changed: fn(*mut ConversationContentQObject),
    pub(super) expiration_period_changed: fn(*mut ConversationContentQObject),
    pub(super) muted_changed: fn(*mut ConversationContentQObject),
    pub(super) pairwise_changed: fn(*mut ConversationContentQObject),
    pub(super) picture_changed: fn(*mut ConversationContentQObject),
    pub(super) status_changed: fn(*mut ConversationContentQObject),
    pub(super) title_changed: fn(*mut ConversationContentQObject),
    pub(super) try_poll: fn(*mut ConversationContentQObject),
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
            conversation_color_changed: self.conversation_color_changed,
            conversation_id_changed: self.conversation_id_changed,
            expiration_period_changed: self.expiration_period_changed,
            muted_changed: self.muted_changed,
            pairwise_changed: self.pairwise_changed,
            picture_changed: self.picture_changed,
            status_changed: self.status_changed,
            title_changed: self.title_changed,
            try_poll: self.try_poll,
        }
    }

    pub fn clear(&self) {
        let n: *const ConversationContentQObject = null();
        self.qobject
            .store(n as *mut ConversationContentQObject, Ordering::SeqCst);
    }

    pub fn conversation_color_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.conversation_color_changed)(ptr);
        }
    }

    pub fn conversation_id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.conversation_id_changed)(ptr);
        }
    }

    pub fn expiration_period_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.expiration_period_changed)(ptr);
        }
    }

    pub fn muted_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.muted_changed)(ptr);
        }
    }

    pub fn pairwise_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.pairwise_changed)(ptr);
        }
    }

    pub fn picture_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.picture_changed)(ptr);
        }
    }

    pub fn status_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.status_changed)(ptr);
        }
    }

    pub fn title_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.title_changed)(ptr);
        }
    }

    pub fn try_poll(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.try_poll)(ptr);
        }
    }
}

pub trait ConversationContentTrait {
    fn new(
        emit: ConversationContentEmitter,
        members: Members,
        messages: Messages,
    ) -> Self;

    fn emit(&mut self) -> &mut ConversationContentEmitter;

    fn conversation_color(&self) -> u32;

    fn conversation_id(&self) -> Option<&[u8]>;

    fn set_conversation_id(
        &mut self,
        value: Option<&[u8]>,
    );

    fn expiration_period(&self) -> u8;

    fn set_expiration_period(
        &mut self,
        value: u8,
    );

    fn members(&self) -> &Members;

    fn members_mut(&mut self) -> &mut Members;

    fn messages(&self) -> &Messages;

    fn messages_mut(&mut self) -> &mut Messages;

    fn muted(&self) -> bool;

    fn set_muted(
        &mut self,
        value: bool,
    );

    fn pairwise(&self) -> bool;

    fn picture(&self) -> Option<String>;

    fn status(&self) -> u8;

    fn set_status(
        &mut self,
        value: u8,
    );

    fn title(&self) -> Option<String>;

    fn set_title(
        &mut self,
        value: Option<String>,
    );

    fn poll_update(&mut self) -> ();

    fn set_picture(
        &mut self,
        picture: String,
    ) -> ();
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
        conversation_content_conversation_color_changed,
        conversation_content_conversation_id_changed,
        conversation_content_expiration_period_changed,
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
        members_new_typing_indicator,
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
        builder_expiration_period_changed,
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
        builder_op_aux_content_changed,
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
        messages_last_msg_digest_changed,
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
        conversation_content_muted_changed,
        conversation_content_pairwise_changed,
        conversation_content_picture_changed,
        conversation_content_status_changed,
        conversation_content_title_changed,
        conversation_content_try_poll,
    } = ptr_bundle;
    let members_emit = MembersEmitter {
        qobject: Arc::new(AtomicPtr::new(members)),
        filter_changed: members_filter_changed,
        filter_regex_changed: members_filter_regex_changed,
        new_data_ready: members_new_data_ready,
        new_typing_indicator: members_new_typing_indicator,
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
        expiration_period_changed: builder_expiration_period_changed,
        has_doc_attachment_changed: builder_has_doc_attachment_changed,
        has_media_attachment_changed: builder_has_media_attachment_changed,
        is_reply_changed: builder_is_reply_changed,
        op_author_changed: builder_op_author_changed,
        op_aux_content_changed: builder_op_aux_content_changed,
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
        last_msg_digest_changed: messages_last_msg_digest_changed,
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
        conversation_color_changed: conversation_content_conversation_color_changed,
        conversation_id_changed: conversation_content_conversation_id_changed,
        expiration_period_changed: conversation_content_expiration_period_changed,
        muted_changed: conversation_content_muted_changed,
        pairwise_changed: conversation_content_pairwise_changed,
        picture_changed: conversation_content_picture_changed,
        status_changed: conversation_content_status_changed,
        title_changed: conversation_content_title_changed,
        try_poll: conversation_content_try_poll,
    };
    let d_conversation_content =
        ConversationContent::new(conversation_content_emit, d_members, d_messages);
    d_conversation_content
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_free(ptr: *mut ConversationContent) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_poll_update(ptr: *mut ConversationContent) {
    let obj = &mut *ptr;
    obj.poll_update()
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_set_picture(
    ptr: *mut ConversationContent,
    picture_str: *const c_ushort,
    picture_len: c_int,
) {
    let obj = &mut *ptr;
    let mut picture = String::new();
    set_string_from_utf16(&mut picture, picture_str, picture_len);
    obj.set_picture(picture)
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_conversation_color_get(
    ptr: *const ConversationContent
) -> u32 {
    (&*ptr).conversation_color()
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
pub unsafe extern "C" fn conversation_content_expiration_period_get(
    ptr: *const ConversationContent
) -> u8 {
    (&*ptr).expiration_period()
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_expiration_period_set(
    ptr: *mut ConversationContent,
    value: u8,
) {
    (&mut *ptr).set_expiration_period(value)
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
pub unsafe extern "C" fn conversation_content_muted_get(ptr: *const ConversationContent) -> bool {
    (&*ptr).muted()
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_muted_set(
    ptr: *mut ConversationContent,
    value: bool,
) {
    (&mut *ptr).set_muted(value)
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_pairwise_get(
    ptr: *const ConversationContent
) -> bool {
    (&*ptr).pairwise()
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_picture_get(
    ptr: *const ConversationContent,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.picture();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_status_get(ptr: *const ConversationContent) -> u8 {
    (&*ptr).status()
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_status_set(
    ptr: *mut ConversationContent,
    value: u8,
) {
    (&mut *ptr).set_status(value)
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_title_get(
    ptr: *const ConversationContent,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.title();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_title_set(
    ptr: *mut ConversationContent,
    value: *const c_ushort,
    len: c_int,
) {
    let obj = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, value, len);
    obj.set_title(Some(s));
}

#[no_mangle]
pub unsafe extern "C" fn conversation_content_title_set_none(ptr: *mut ConversationContent) {
    let obj = &mut *ptr;
    obj.set_title(None);
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ConversationContentPtrBundle {
    conversation_content: *mut ConversationContentQObject,
    conversation_content_conversation_color_changed: fn(*mut ConversationContentQObject),
    conversation_content_conversation_id_changed: fn(*mut ConversationContentQObject),
    conversation_content_expiration_period_changed: fn(*mut ConversationContentQObject),
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
    members_new_typing_indicator: fn(*mut MembersQObject),
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
    builder_expiration_period_changed: fn(*mut MessageBuilderQObject),
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
    builder_op_aux_content_changed: fn(*mut MessageBuilderQObject),
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
    messages_last_msg_digest_changed: fn(*mut MessagesQObject),
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
    conversation_content_muted_changed: fn(*mut ConversationContentQObject),
    conversation_content_pairwise_changed: fn(*mut ConversationContentQObject),
    conversation_content_picture_changed: fn(*mut ConversationContentQObject),
    conversation_content_status_changed: fn(*mut ConversationContentQObject),
    conversation_content_title_changed: fn(*mut ConversationContentQObject),
    conversation_content_try_poll: fn(*mut ConversationContentQObject),
}
