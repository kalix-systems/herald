use crate::interface::{ReplyWidthCalcEmitter as Emit, ReplyWidthCalcTrait as Interface};

const IMG_WIDTH: f64 = 300.0;
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
        image_attach: bool,
        max_bubble_width: f64,
        message_label_width: f64,
        message_body_width: f64,
        unknown_body_width: f64,
    ) -> f64 {
        if image_attach {
            return IMG_WIDTH;
        }

        let body_max = message_label_width.max(message_body_width);

        if unknown_body_width > message_body_width {
            unknown_body_width.max(body_max).min(max_bubble_width)
        } else {
            body_max
        }
    }

    fn doc(
        &self,
        image_attach: bool,
        bubble_max_width: f64,
        reply_label_width: f64,
        message_label_width: f64,
        message_body_width: f64,
        reply_body_width: f64,
        stamp_width: f64,
        reply_ts_width: f64,
        file_clip_width: f64,
    ) -> f64 {
        if image_attach {
            return IMG_WIDTH;
        }

        let reply_width = reply_label_width.max(reply_body_width).max(reply_ts_width);
        let message_width = message_label_width.max(message_body_width).max(stamp_width);

        let content_max = message_width.max(reply_width).max(file_clip_width);

        SENSIBLE_MIN.max(content_max.min(bubble_max_width))
    }
}
