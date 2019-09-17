use crate::interface::*;

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

    fn chat_bubble_natural_width(&self, chat_pane_width: f64, text_width: f64) -> f64 {
        (chat_pane_width * 2. / 3.).min(text_width)
    }

    fn emit(&mut self) -> &mut HeraldUtilsEmitter {
        &mut self.emit
    }
}
