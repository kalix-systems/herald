use super::*;

pub struct DocumentAttachmentsQObject;

pub struct DocumentAttachmentsEmitter {
    pub(super) qobject: Arc<AtomicPtr<DocumentAttachmentsQObject>>,
    pub(super) new_data_ready: fn(*mut DocumentAttachmentsQObject),
}

impl DocumentAttachmentsEmitter {
    /// Clone the emitter
    /// 
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> DocumentAttachmentsEmitter {
        DocumentAttachmentsEmitter {
            qobject: self.qobject.clone(),
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const DocumentAttachmentsQObject = null();
        self.qobject.store(n as *mut DocumentAttachmentsQObject, Ordering::SeqCst);
    }

    pub fn new_data_ready(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.new_data_ready)(ptr);
        }
    }
}

#[derive(Clone)]
pub struct DocumentAttachmentsList {
    pub(super) qobject: *mut DocumentAttachmentsQObject,
    pub(super) layout_about_to_be_changed: fn(*mut DocumentAttachmentsQObject),
    pub(super) layout_changed: fn(*mut DocumentAttachmentsQObject),
    pub(super) begin_reset_model: fn(*mut DocumentAttachmentsQObject),
    pub(super) end_reset_model: fn(*mut DocumentAttachmentsQObject),
    pub(super) end_insert_rows: fn(*mut DocumentAttachmentsQObject),
    pub(super) end_move_rows: fn(*mut DocumentAttachmentsQObject),
    pub(super) end_remove_rows: fn(*mut DocumentAttachmentsQObject),
    pub(super) begin_insert_rows: fn(*mut DocumentAttachmentsQObject,  usize, usize),
    pub(super) begin_remove_rows: fn(*mut DocumentAttachmentsQObject,  usize, usize),
    pub(super) data_changed: fn(*mut DocumentAttachmentsQObject, usize, usize),
    pub(super) begin_move_rows: fn(*mut DocumentAttachmentsQObject, usize, usize, usize),
}

impl DocumentAttachmentsList {
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

pub trait DocumentAttachmentsTrait {
    fn new(emit: DocumentAttachmentsEmitter, model: DocumentAttachmentsList) -> Self;

    fn emit(&mut self) -> &mut DocumentAttachmentsEmitter;

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

    fn document_attachment_name(&self, index: usize) -> String;

    fn document_attachment_size(&self, index: usize) -> u64;
}

#[no_mangle]
pub unsafe extern "C" fn document_attachments_new(ptr_bundle: *mut DocumentAttachmentsPtrBundle) -> *mut DocumentAttachments {
    let d_document_attachments = document_attachments_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_document_attachments))
}

pub unsafe fn document_attachments_new_inner(ptr_bundle: *mut DocumentAttachmentsPtrBundle) -> DocumentAttachments {
    let ptr_bundle = *ptr_bundle;

    let DocumentAttachmentsPtrBundle {
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
    d_document_attachments
}

#[no_mangle]
pub unsafe extern "C" fn document_attachments_free(ptr: *mut DocumentAttachments) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn document_attachments_row_count(ptr: *const DocumentAttachments) -> c_int {
    to_c_int((&*ptr).row_count())
}

#[no_mangle]
pub unsafe extern "C" fn document_attachments_insert_rows(ptr: *mut DocumentAttachments, row: c_int, count: c_int) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => 
        {
            (&mut *ptr).insert_rows(row, count)
        }
        _ => false
    }
}

#[no_mangle]
pub unsafe extern "C" fn document_attachments_remove_rows(ptr: *mut DocumentAttachments, row: c_int, count: c_int) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => 
        {
            (&mut *ptr).remove_rows(row, count)
        }
        _ => false
    }
}

#[no_mangle]
pub unsafe extern "C" fn document_attachments_can_fetch_more(ptr: *const DocumentAttachments) -> bool {
    (&*ptr).can_fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn document_attachments_fetch_more(ptr: *mut DocumentAttachments) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn document_attachments_sort(ptr: *mut DocumentAttachments, column: u8, order: SortOrder) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn document_attachments_data_document_attachment_name(ptr: *const DocumentAttachments, row: c_int, d: *mut QString, set: fn(*mut QString, *const c_char, len: c_int)) {
    let obj = &*ptr;
    let data = obj.document_attachment_name(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn document_attachments_data_document_attachment_size(ptr: *const DocumentAttachments, row: c_int) -> u64 {
    let obj = &*ptr;
    obj.document_attachment_size(to_usize(row).unwrap_or(0))
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct DocumentAttachmentsPtrBundle {
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
}
