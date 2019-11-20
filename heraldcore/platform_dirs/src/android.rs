use lazy_static::*;
use std::path::PathBuf;

lazy_static! {
    pub static ref DATA_DIR: PathBuf = "".into();
}
