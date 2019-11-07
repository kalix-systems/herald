use lazy_static::*;
use std::{fs::create_dir_all, path::PathBuf};

#[cfg_attr(
    not(any(target_os = "android", target_os = "ios")),
    path = "desktop.rs"
)]
#[cfg_attr(target_os = "android", path = "android.rs")]
#[cfg_attr(target_os = "ios", path = "ios.rs")]
mod imp;

lazy_static! {
    pub static ref DB_DIR: PathBuf = db_dir();
    pub static ref PROFILE_PICTURES_DIR: PathBuf = profile_pictures_dir();
    pub static ref ATTACHMENTS_DIR: PathBuf = attachments_dir();
}

use imp::DATA_DIR;

fn db_dir() -> PathBuf {
    let db_dir = DATA_DIR.join("db");
    if let Err(e) = create_dir_all(&db_dir) {
        eprintln!("Error creating database directory: {}", e);
    }

    db_dir
}

fn profile_pictures_dir() -> PathBuf {
    let pic_dir = DATA_DIR.join("profile_pictures");

    if let Err(e) = create_dir_all(&pic_dir) {
        eprintln!("Error creating profile picture directory: {}", e);
    }

    pic_dir
}

fn attachments_dir() -> PathBuf {
    let attachments_dir = DATA_DIR.join("attachments");

    if let Err(e) = create_dir_all(&attachments_dir) {
        eprintln!("Error creating attachments directory: {}", e);
    }

    attachments_dir
}
