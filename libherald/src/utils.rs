use std::fs::remove_file;

pub(crate) fn delete_db() {
    remove_file("store.sqlite3").ok();
}
