use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

static DIR: once_cell::sync::OnceCell<PathBuf> = once_cell::sync::OnceCell::new();

static FALLBACK_DIR: &str = ".data_dir";

#[cfg(not(feature = "deploy_desktop"))]
#[must_use]
pub fn set_data_dir<P: AsRef<Path>>(_: P) -> Option<()> {
    DIR.set(PathBuf::from(FALLBACK_DIR)).ok()
}

#[cfg(feature = "deploy_desktop")]
#[must_use]
pub fn set_data_dir<P: AsRef<Path>>(path: P) -> Option<()> {
    DIR.set(path.as_ref().to_path_buf()).ok()
}

#[cfg(feature = "deploy_desktop")]
fn data_dir() -> PathBuf {
    DIR.get()
        .unwrap_or(&PathBuf::from(FALLBACK_DIR))
        .to_path_buf()
}

#[cfg(not(feature = "deploy_desktop"))]
fn data_dir() -> PathBuf {
    PathBuf::from(FALLBACK_DIR)
}

pub fn db_dir() -> PathBuf {
    let db_dir = data_dir().join("db");

    drop(create_dir_all(&db_dir));

    db_dir
}

pub fn pictures_dir() -> PathBuf {
    let pic_dir = data_dir().join("pictures");

    drop(create_dir_all(&pic_dir));

    pic_dir
}

pub fn attachments_dir() -> PathBuf {
    let attachments_dir = data_dir().join("attachments");

    drop(create_dir_all(&attachments_dir));

    attachments_dir
}
