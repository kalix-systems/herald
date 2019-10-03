use crate::interface::*;

/// A collection of pure functions that are used in QML.
pub struct HeraldUtils {
    emit: HeraldUtilsEmitter,
}

impl HeraldUtilsTrait for HeraldUtils {
    fn new(emit: HeraldUtilsEmitter) -> Self {
        HeraldUtils { emit }
    }

    fn compare_byte_array(&self, bs1: &[u8], bs2: &[u8]) -> bool {
        bs1 == bs2
    }

    fn is_valid_rand_id(&self, bs: &[u8]) -> bool {
        bs.len() == 32
    }

    fn chat_bubble_natural_width(&self, chat_pane_width: f64, text_width: f64) -> f64 {
        (chat_pane_width * 2. / 3.).min(text_width)
    }

    fn emit(&mut self) -> &mut HeraldUtilsEmitter {
        &mut self.emit
    }
}
