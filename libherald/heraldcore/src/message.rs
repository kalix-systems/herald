use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use rusqlite::NO_PARAMS;

#[derive(Default)]
pub struct Messages {}

impl DBTable for Messages {
    fn create_table(&self) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/message/create_table.sql"), NO_PARAMS)?;

        Ok(())
    }

    fn drop_table(&self) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/message/drop_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists(&self) -> Result<bool, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/message/table_exists.sql"))?;
        Ok(stmt.exists(NO_PARAMS)?)
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
        assert!(messages.exists().unwrap());
        messages.create_table().unwrap();
        assert!(messages.exists().unwrap());
        messages.drop_table().unwrap();
        assert!(!messages.exists().unwrap());
    }
}
