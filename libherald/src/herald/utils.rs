use crate::interface::*;

/// A collection of pure functions that are used in QML.
pub struct Utils {
    emit: UtilsEmitter,
}

impl UtilsTrait for Utils {
    fn new(emit: UtilsEmitter) -> Self {
        Utils { emit }
    }

    fn compare_byte_array(
        &self,
        bs1: &[u8],
        bs2: &[u8],
    ) -> bool {
        bs1 == bs2
    }

    fn is_valid_rand_id(
        &self,
        bs: &[u8],
    ) -> bool {
        bs.len() == 32
    }

    fn emit(&mut self) -> &mut UtilsEmitter {
        &mut self.emit
    }
}
