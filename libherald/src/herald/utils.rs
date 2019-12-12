use crate::interface::*;
use crate::utils::strip_qrc;
use crate::{err, none};
use std::fs::copy;
use std::path::Path;
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

    fn save_file(
        &self,
        fpath: String,
        target_path: String,
    ) -> bool {
        if let Some(target_path) = strip_qrc(target_path) {
            let existing_path = Path::new(&fpath);
            let fname = none!(existing_path.file_name(), false);
            let target_path = Path::new(&target_path).join(&fname);
            err!(copy(existing_path, target_path), false);
            true
        } else {
            false
        }
    }

    fn emit(&mut self) -> &mut UtilsEmitter {
        &mut self.emit
    }
}
