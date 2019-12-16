use crate::interface::{ReplyWidthCalcEmitter as Emit, ReplyWidthCalcTrait as Interface};

const SENSIBLE_MIN: f64 = 150.0;

/// Reply width calculator
pub struct ReplyWidthCalc(Emit);

impl Interface for ReplyWidthCalc {
    fn new(emit: Emit) -> Self {
        Self(emit)
    }

    fn emit(&mut self) -> &mut Emit {
        &mut self.0
    }

    fn unknown(
        &self,
        bubble_max_width: f64,
        message_label_width: f64,
        message_body_width: f64,
        unknown_body_width: f64,
    ) -> f64 {
        let body_max = message_label_width.max(message_body_width);

        if unknown_body_width > message_body_width {
            unknown_body_width.max(body_max).min(bubble_max_width)
        } else {
            body_max
        }
    }

    fn text(
        &self,
        bubble_max_width: f64,
        message_label_width: f64,
        message_body_width: f64,
        stamp_width: f64,
        reply_label_width: f64,
        reply_body_width: f64,
        reply_ts_width: f64,
    ) -> f64 {
        let reply_width = reply_label_width.max(reply_body_width).max(reply_ts_width);
        let message_width = message_label_width.max(message_body_width).max(stamp_width);

        let content_max = message_width.max(reply_width);

        bubble_max_width.min(content_max)
    }

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
    ) -> f64 {
        let reply_width = reply_label_width.max(reply_body_width).max(reply_ts_width);
        let message_width = message_label_width.max(message_body_width).max(stamp_width);

        let content_max = message_width.max(reply_width).max(reply_file_clip_width);

        SENSIBLE_MIN.max(content_max.min(bubble_max_width))
    }
}
