use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use rusqlite::NO_PARAMS;

#[derive(Default)]
pub struct Messages {
    db: Database,
}

impl DBTable for Messages {
    fn create_table(&self) -> Result<(), HErr> {
        let db = &self.db;
        db.execute(include_str!("sql/message/create_table.sql"), NO_PARAMS)?;

        Ok(())
    }

    fn drop_table(&self) -> Result<(), HErr> {
        let db = &self.db;
        db.execute(include_str!("sql/message/drop_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists(&self) -> bool {
        let db = &self.db;
        if let Ok(mut stmt) = db.prepare(include_str!("sql/message/table_exists.sql")) {
            stmt.exists(NO_PARAMS).unwrap_or(false)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test_derive::serial;

    #[test]
    #[serial]
    fn create_drop_exists() {
        let messages = Messages::default();
        // drop twice, it shouldn't panic on multiple drops
        messages.drop_table().unwrap();
        messages.drop_table().unwrap();

        messages.create_table().unwrap();
        assert!(messages.exists());
        messages.create_table().unwrap();
        assert!(messages.exists());
        messages.drop_table().unwrap();
        assert!(!messages.exists());
    }
}
