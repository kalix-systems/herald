use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use rusqlite::NO_PARAMS;

#[derive(Default)]
pub struct Messages {}

impl DBTable for Messages {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/message/create_table.sql"), NO_PARAMS)?;

        Ok(())
    }

    fn drop_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/message/drop_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists() -> Result<bool, HErr> {
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
        // drop twice, it shouldn't panic on multiple drops
        Messages::drop_table().unwrap();
        Messages::drop_table().unwrap();

        Messages::create_table().unwrap();
        assert!(Messages::exists().unwrap());
        Messages::create_table().unwrap();
        assert!(Messages::exists().unwrap());
        Messages::drop_table().unwrap();
        assert!(!Messages::exists().unwrap());
    }
}
