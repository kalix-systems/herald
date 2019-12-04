use super::*;

pub struct AttachmentsQObject;

pub struct AttachmentsEmitter {
    pub(super) qobject: Arc<AtomicPtr<AttachmentsQObject>>,
    pub(super) attachments_msg_id_changed: fn(*mut AttachmentsQObject),
    pub(super) new_data_ready: fn(*mut AttachmentsQObject),
}

impl AttachmentsEmitter {
    /// Clone the emitter
    /// 
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> AttachmentsEmitter {
        AttachmentsEmitter {
            qobject: self.qobject.clone(),
            attachments_msg_id_changed: self.attachments_msg_id_changed,
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const AttachmentsQObject = null();
        self.qobject.store(n as *mut AttachmentsQObject, Ordering::SeqCst);
    }

    pub fn attachments_msg_id_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.attachments_msg_id_changed)(ptr);
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
pub struct AttachmentsList {
    pub(super) qobject: *mut AttachmentsQObject,
    pub(super) layout_about_to_be_changed: fn(*mut AttachmentsQObject),
    pub(super) layout_changed: fn(*mut AttachmentsQObject),
    pub(super) begin_reset_model: fn(*mut AttachmentsQObject),
    pub(super) end_reset_model: fn(*mut AttachmentsQObject),
    pub(super) end_insert_rows: fn(*mut AttachmentsQObject),
    pub(super) end_move_rows: fn(*mut AttachmentsQObject),
    pub(super) end_remove_rows: fn(*mut AttachmentsQObject),
    pub(super) begin_insert_rows: fn(*mut AttachmentsQObject,  usize, usize),
    pub(super) begin_remove_rows: fn(*mut AttachmentsQObject,  usize, usize),
    pub(super) data_changed: fn(*mut AttachmentsQObject, usize, usize),
    pub(super) begin_move_rows: fn(*mut AttachmentsQObject, usize, usize, usize),
}

impl AttachmentsList {
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

pub trait AttachmentsTrait {
    fn new(emit: AttachmentsEmitter, model: AttachmentsList, document_attachments: DocumentAttachments, media_attachments: MediaAttachments) -> Self;

    fn emit(&mut self) -> &mut AttachmentsEmitter;

    fn attachments_msg_id(&self) -> Option<&[u8]>;

    fn set_attachments_msg_id(&mut self, value: Option<&[u8]>);

    fn document_attachments(&self) -> &DocumentAttachments;

    fn document_attachments_mut(&mut self) -> &mut DocumentAttachments;

    fn media_attachments(&self) -> &MediaAttachments;

    fn media_attachments_mut(&mut self) -> &mut MediaAttachments;

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
}

#[no_mangle]
pub unsafe extern "C" fn attachments_new(ptr_bundle: *mut AttachmentsPtrBundle) -> *mut Attachments {
    let d_attachments = attachments_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_attachments))
}

pub unsafe fn attachments_new_inner(ptr_bundle: *mut AttachmentsPtrBundle) -> Attachments {
    let ptr_bundle = *ptr_bundle;

    let AttachmentsPtrBundle {
        attachments
        ,
        attachments_attachments_msg_id_changed,
        document_attachments
        ,
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
        media_attachments
        ,
        media_attachments_media_attachment_four_changed,
        media_attachments_media_attachment_one_changed,
        media_attachments_media_attachment_three_changed,
        media_attachments_media_attachment_two_changed,
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
        attachments_new_data_ready,
        attachments_layout_about_to_be_changed,
        attachments_layout_changed,
        attachments_data_changed,
        attachments_begin_reset_model,
        attachments_end_reset_model,
        attachments_begin_insert_rows,
        attachments_end_insert_rows,
        attachments_begin_move_rows,
        attachments_end_move_rows,
        attachments_begin_remove_rows,
        attachments_end_remove_rows,
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
    let d_document_attachments = DocumentAttachments::new(document_attachments_emit, model
    );
    let media_attachments_emit = MediaAttachmentsEmitter {
        qobject: Arc::new(AtomicPtr::new(media_attachments)),
        media_attachment_four_changed: media_attachments_media_attachment_four_changed,
        media_attachment_one_changed: media_attachments_media_attachment_one_changed,
        media_attachment_three_changed: media_attachments_media_attachment_three_changed,
        media_attachment_two_changed: media_attachments_media_attachment_two_changed,
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
    let d_media_attachments = MediaAttachments::new(media_attachments_emit, model
    );
    let attachments_emit = AttachmentsEmitter {
        qobject: Arc::new(AtomicPtr::new(attachments)),
        attachments_msg_id_changed: attachments_attachments_msg_id_changed,
        new_data_ready: attachments_new_data_ready,
    };
    let model = AttachmentsList {

                qobject: attachments,
                layout_about_to_be_changed: attachments_layout_about_to_be_changed,
                layout_changed: attachments_layout_changed,
                data_changed: attachments_data_changed,
                begin_reset_model: attachments_begin_reset_model,
                end_reset_model: attachments_end_reset_model,
                begin_insert_rows: attachments_begin_insert_rows,
                end_insert_rows: attachments_end_insert_rows,
                begin_move_rows: attachments_begin_move_rows,
                end_move_rows: attachments_end_move_rows,
                begin_remove_rows: attachments_begin_remove_rows,
                end_remove_rows: attachments_end_remove_rows,
                
    };
    let d_attachments = Attachments::new(attachments_emit, model
    , d_document_attachments
    , d_media_attachments
    );
    d_attachments
}

#[no_mangle]
pub unsafe extern "C" fn attachments_free(ptr: *mut Attachments) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn attachments_attachments_msg_id_get(ptr: *const Attachments, prop: *mut QByteArray, set: fn(*mut QByteArray, *const c_char, c_int)) {
    let obj = &*ptr;
    let value = obj.attachments_msg_id();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn attachments_attachments_msg_id_set(ptr: *mut Attachments, value: *const c_char, len: c_int) {
    let obj = &mut *ptr;
    let value = qba_slice!(value, len);
    obj.set_attachments_msg_id(Some(value));
}

#[no_mangle]
pub unsafe extern "C" fn attachments_attachments_msg_id_set_none(ptr: *mut Attachments) {
    let obj = &mut *ptr;
    obj.set_attachments_msg_id(None);
}

#[no_mangle]
pub unsafe extern "C" fn attachments_document_attachments_get(ptr: *mut Attachments) -> *mut DocumentAttachments {
    (&mut *ptr).document_attachments_mut()
}

#[no_mangle]
pub unsafe extern "C" fn attachments_media_attachments_get(ptr: *mut Attachments) -> *mut MediaAttachments {
    (&mut *ptr).media_attachments_mut()
}

#[no_mangle]
pub unsafe extern "C" fn attachments_row_count(ptr: *const Attachments) -> c_int {
    to_c_int((&*ptr).row_count())
}

#[no_mangle]
pub unsafe extern "C" fn attachments_insert_rows(ptr: *mut Attachments, row: c_int, count: c_int) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => 
        {
            (&mut *ptr).insert_rows(row, count)
        }
        _ => false
    }
}

#[no_mangle]
pub unsafe extern "C" fn attachments_remove_rows(ptr: *mut Attachments, row: c_int, count: c_int) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => 
        {
            (&mut *ptr).remove_rows(row, count)
        }
        _ => false
    }
}

#[no_mangle]
pub unsafe extern "C" fn attachments_can_fetch_more(ptr: *const Attachments) -> bool {
    (&*ptr).can_fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn attachments_fetch_more(ptr: *mut Attachments) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn attachments_sort(ptr: *mut Attachments, column: u8, order: SortOrder) {
    (&mut *ptr).sort(column, order)
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentsPtrBundle {
    attachments: *mut AttachmentsQObject,
    attachments_attachments_msg_id_changed: fn(*mut AttachmentsQObject),
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
    media_attachments: *mut MediaAttachmentsQObject,
    media_attachments_media_attachment_four_changed: fn(*mut MediaAttachmentsQObject),
    media_attachments_media_attachment_one_changed: fn(*mut MediaAttachmentsQObject),
    media_attachments_media_attachment_three_changed: fn(*mut MediaAttachmentsQObject),
    media_attachments_media_attachment_two_changed: fn(*mut MediaAttachmentsQObject),
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
    attachments_new_data_ready: fn(*mut AttachmentsQObject),
    attachments_layout_about_to_be_changed: fn(*mut AttachmentsQObject),
    attachments_layout_changed: fn(*mut AttachmentsQObject),
    attachments_data_changed: fn(*mut AttachmentsQObject, usize, usize),
    attachments_begin_reset_model: fn(*mut AttachmentsQObject),
    attachments_end_reset_model: fn(*mut AttachmentsQObject),
    attachments_begin_insert_rows: fn(*mut AttachmentsQObject, usize, usize),
    attachments_end_insert_rows: fn(*mut AttachmentsQObject),
    attachments_begin_move_rows: fn(*mut AttachmentsQObject, usize, usize, usize),
    attachments_end_move_rows: fn(*mut AttachmentsQObject),
    attachments_begin_remove_rows: fn(*mut AttachmentsQObject, usize, usize),
    attachments_end_remove_rows: fn(*mut AttachmentsQObject),
}
