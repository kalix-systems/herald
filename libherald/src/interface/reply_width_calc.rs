use super::*;

pub struct ReplyWidthCalcQObject;

pub struct ReplyWidthCalcEmitter {
    pub(super) qobject: Arc<AtomicPtr<ReplyWidthCalcQObject>>,
}

impl ReplyWidthCalcEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> ReplyWidthCalcEmitter {
        ReplyWidthCalcEmitter {
            qobject: self.qobject.clone(),
        }
    }

    pub fn clear(&self) {
        let n: *const ReplyWidthCalcQObject = null();
        self.qobject
            .store(n as *mut ReplyWidthCalcQObject, Ordering::SeqCst);
    }
}

pub trait ReplyWidthCalcTrait {
    fn new(emit: ReplyWidthCalcEmitter) -> Self;

    fn emit(&mut self) -> &mut ReplyWidthCalcEmitter;

    fn doc(
        &self,
        bubble_max_width: f64,
        message_label_width: f64,
        message_body_width: f64,
        stamp_width: f64,
        reply_label_width: f64,
        reply_body_width: f64,
        reply_ts_width: f64,
        reply_file_clip_width: f64,
    ) -> f64;

    fn text(
        &self,
        bubble_max_width: f64,
        message_label_width: f64,
        message_body_width: f64,
        stamp_width: f64,
        reply_label_width: f64,
        reply_body_width: f64,
        reply_ts_width: f64,
    ) -> f64;

    fn unknown(
        &self,
        bubble_max_width: f64,
        message_label_width: f64,
        message_body_width: f64,
        unknown_body_width: f64,
    ) -> f64;
}

#[no_mangle]
pub unsafe extern "C" fn reply_width_calc_new(
    ptr_bundle: *mut ReplyWidthCalcPtrBundle
) -> *mut ReplyWidthCalc {
    let d_reply_width_calc = reply_width_calc_new_inner(ptr_bundle);
    Box::into_raw(Box::new(d_reply_width_calc))
}

pub unsafe fn reply_width_calc_new_inner(
    ptr_bundle: *mut ReplyWidthCalcPtrBundle
) -> ReplyWidthCalc {
    let ptr_bundle = *ptr_bundle;

    let ReplyWidthCalcPtrBundle { reply_width_calc } = ptr_bundle;
    let reply_width_calc_emit = ReplyWidthCalcEmitter {
        qobject: Arc::new(AtomicPtr::new(reply_width_calc)),
    };
    let d_reply_width_calc = ReplyWidthCalc::new(reply_width_calc_emit);
    d_reply_width_calc
}

#[no_mangle]
pub unsafe extern "C" fn reply_width_calc_free(ptr: *mut ReplyWidthCalc) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn reply_width_calc_doc(
    ptr: *const ReplyWidthCalc,
    bubble_max_width: f64,
    message_label_width: f64,
    message_body_width: f64,
    stamp_width: f64,
    reply_label_width: f64,
    reply_body_width: f64,
    reply_ts_width: f64,
    reply_file_clip_width: f64,
) -> f64 {
    let obj = &*ptr;
    obj.doc(
        bubble_max_width,
        message_label_width,
        message_body_width,
        stamp_width,
        reply_label_width,
        reply_body_width,
        reply_ts_width,
        reply_file_clip_width,
    )
}

#[no_mangle]
pub unsafe extern "C" fn reply_width_calc_text(
    ptr: *const ReplyWidthCalc,
    bubble_max_width: f64,
    message_label_width: f64,
    message_body_width: f64,
    stamp_width: f64,
    reply_label_width: f64,
    reply_body_width: f64,
    reply_ts_width: f64,
) -> f64 {
    let obj = &*ptr;
    obj.text(
        bubble_max_width,
        message_label_width,
        message_body_width,
        stamp_width,
        reply_label_width,
        reply_body_width,
        reply_ts_width,
    )
}

#[no_mangle]
pub unsafe extern "C" fn reply_width_calc_unknown(
    ptr: *const ReplyWidthCalc,
    bubble_max_width: f64,
    message_label_width: f64,
    message_body_width: f64,
    unknown_body_width: f64,
) -> f64 {
    let obj = &*ptr;
    obj.unknown(
        bubble_max_width,
        message_label_width,
        message_body_width,
        unknown_body_width,
    )
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ReplyWidthCalcPtrBundle {
    reply_width_calc: *mut ReplyWidthCalcQObject,
}
