use super::*;

pub struct MediaAttachmentsQObject;

pub struct MediaAttachmentsEmitter {
    pub(super) qobject: Arc<AtomicPtr<MediaAttachmentsQObject>>,
    pub(super) media_attachment_four_changed: fn(*mut MediaAttachmentsQObject),
    pub(super) media_attachment_one_changed: fn(*mut MediaAttachmentsQObject),
    pub(super) media_attachment_three_changed: fn(*mut MediaAttachmentsQObject),
    pub(super) media_attachment_two_changed: fn(*mut MediaAttachmentsQObject),
    pub(super) new_data_ready: fn(*mut MediaAttachmentsQObject),
}

impl MediaAttachmentsEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> MediaAttachmentsEmitter {
        MediaAttachmentsEmitter {
            qobject: self.qobject.clone(),
            media_attachment_four_changed: self.media_attachment_four_changed,
            media_attachment_one_changed: self.media_attachment_one_changed,
            media_attachment_three_changed: self.media_attachment_three_changed,
            media_attachment_two_changed: self.media_attachment_two_changed,
            new_data_ready: self.new_data_ready,
        }
    }

    pub fn clear(&self) {
        let n: *const MediaAttachmentsQObject = null();
        self.qobject
            .store(n as *mut MediaAttachmentsQObject, Ordering::SeqCst);
    }

    pub fn media_attachment_four_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.media_attachment_four_changed)(ptr);
        }
    }

    pub fn media_attachment_one_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.media_attachment_one_changed)(ptr);
        }
    }

    pub fn media_attachment_three_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.media_attachment_three_changed)(ptr);
        }
    }

    pub fn media_attachment_two_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);

        if !ptr.is_null() {
            (self.media_attachment_two_changed)(ptr);
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
pub struct MediaAttachmentsList {
    pub(super) qobject: *mut MediaAttachmentsQObject,
    pub(super) layout_about_to_be_changed: fn(*mut MediaAttachmentsQObject),
    pub(super) layout_changed: fn(*mut MediaAttachmentsQObject),
    pub(super) begin_reset_model: fn(*mut MediaAttachmentsQObject),
    pub(super) end_reset_model: fn(*mut MediaAttachmentsQObject),
    pub(super) end_insert_rows: fn(*mut MediaAttachmentsQObject),
    pub(super) end_move_rows: fn(*mut MediaAttachmentsQObject),
    pub(super) end_remove_rows: fn(*mut MediaAttachmentsQObject),
    pub(super) begin_insert_rows: fn(*mut MediaAttachmentsQObject, usize, usize),
    pub(super) begin_remove_rows: fn(*mut MediaAttachmentsQObject, usize, usize),
    pub(super) data_changed: fn(*mut MediaAttachmentsQObject, usize, usize),
    pub(super) begin_move_rows: fn(*mut MediaAttachmentsQObject, usize, usize, usize),
}

impl MediaAttachmentsList {
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

pub trait MediaAttachmentsTrait {
    fn new(
        emit: MediaAttachmentsEmitter,
        model: MediaAttachmentsList,
    ) -> Self;

    fn emit(&mut self) -> &mut MediaAttachmentsEmitter;

    fn media_attachment_four(&self) -> Option<&str>;

    fn media_attachment_one(&self) -> Option<&str>;

    fn media_attachment_three(&self) -> Option<&str>;

    fn media_attachment_two(&self) -> Option<&str>;

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

    fn media_attachment_path(
        &self,
        index: usize,
    ) -> &str;
}

#[no_mangle]
pub unsafe extern "C" fn media_attachments_new(
    ptr_bundle: *mut MediaAttachmentsPtrBundle
) -> *mut MediaAttachments {
    let d_media_attachments = media_attachments_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_media_attachments))
}

pub unsafe fn media_attachments_new_inner(
    ptr_bundle: *mut MediaAttachmentsPtrBundle
) -> MediaAttachments {
    let ptr_bundle = *ptr_bundle;

    let MediaAttachmentsPtrBundle {
        media_attachments,
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
    } = ptr_bundle;
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
    let d_media_attachments = MediaAttachments::new(media_attachments_emit, model);
    d_media_attachments
}

#[no_mangle]
pub unsafe extern "C" fn media_attachments_free(ptr: *mut MediaAttachments) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn media_attachments_media_attachment_four_get(
    ptr: *const MediaAttachments,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.media_attachment_four();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn media_attachments_media_attachment_one_get(
    ptr: *const MediaAttachments,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.media_attachment_one();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn media_attachments_media_attachment_three_get(
    ptr: *const MediaAttachments,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.media_attachment_three();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn media_attachments_media_attachment_two_get(
    ptr: *const MediaAttachments,
    prop: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let obj = &*ptr;
    let value = obj.media_attachment_two();
    if let Some(value) = value {
        let str_: *const c_char = value.as_ptr() as (*const c_char);
        set(prop, str_, to_c_int(value.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn media_attachments_row_count(ptr: *const MediaAttachments) -> c_int {
    to_c_int((&*ptr).row_count())
}

#[no_mangle]
pub unsafe extern "C" fn media_attachments_insert_rows(
    ptr: *mut MediaAttachments,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).insert_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn media_attachments_remove_rows(
    ptr: *mut MediaAttachments,
    row: c_int,
    count: c_int,
) -> bool {
    match (to_usize(row), to_usize(count)) {
        (Some(row), Some(count)) => (&mut *ptr).remove_rows(row, count),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn media_attachments_can_fetch_more(ptr: *const MediaAttachments) -> bool {
    (&*ptr).can_fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn media_attachments_fetch_more(ptr: *mut MediaAttachments) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn media_attachments_sort(
    ptr: *mut MediaAttachments,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn media_attachments_data_media_attachment_path(
    ptr: *const MediaAttachments,
    row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let obj = &*ptr;
    let data = obj.media_attachment_path(to_usize(row).unwrap_or(0));
    let str_: *const c_char = data.as_ptr() as *const c_char;
    set(d, str_, to_c_int(data.len()));
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MediaAttachmentsPtrBundle {
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
}
