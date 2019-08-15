use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use rusqlite::NO_PARAMS;

#[derive(Default)]
pub struct Conversations {
    db: Database,
}

impl DBTable for Conversations {
    fn create_table(&mut self) -> Result<(), HErr> {
        let db = &self.db;
        db.execute(include_str!("sql/conversation/create_table.sql"), NO_PARAMS)?;

        Ok(())
    }

    fn drop_table(&mut self) -> Result<(), HErr> {
        let db = &self.db;
        db.execute(include_str!("sql/conversation/drop_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists(&self) -> bool {
        let db = &self.db;
        if let Ok(mut stmt) = db.prepare(include_str!("sql/conversation/table_exists.sql")) {
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
        let mut convs = Conversations::default();
        // drop twice, it shouldn't panic on multiple drops
        convs.drop_table().unwrap();
        convs.drop_table().unwrap();

        convs.create_table().unwrap();
        assert!(convs.exists());
        convs.create_table().unwrap();
        assert!(convs.exists());
        convs.drop_table().unwrap();
        assert!(!convs.exists());
    }
}
