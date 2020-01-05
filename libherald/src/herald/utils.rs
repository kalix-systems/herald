use crate::interface::*;
use crate::utils::strip_qrc;
use crate::{err, none, spawn};
use std::fs;
use std::path::PathBuf;

/// A collection of helper functions that are used in QML.
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
        let target_path = none!(strip_qrc(target_path), false);

        let existing_path = PathBuf::from(fpath);

        let fname = none!(existing_path.file_name(), false);

        let target_path = PathBuf::from(&target_path).join(&fname);

        spawn!(
            {
                err!(fs::copy(existing_path, target_path));
            },
            false
        );
        true
    }

    fn image_dimensions(
        &self,
        path: String,
    ) -> String {
        let path = none!(strip_qrc(path), "".to_owned());
        let (width, height) = err!(
            heraldcore::image_utils::image_dimensions(path),
            "".to_owned()
        );

        (json::object! {
            "width" => width,
            "height" => height,
        })
        .dump()
    }

    fn strip_url_prefix(
        &self,
        url: String,
    ) -> String {
        crate::utils::strip_qrc(url).unwrap_or_default()
    }

    fn image_scaling(
        &self,
        path: String,
        constant: u32,
    ) -> String {
        let dims = err!(
            heraldcore::image_utils::image_scaling(path, constant),
            Default::default()
        );

        json::JsonValue::from(dims).dump()
    }

    fn emit(&mut self) -> &mut UtilsEmitter {
        &mut self.emit
    }
}
