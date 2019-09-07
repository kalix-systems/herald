use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use rusqlite::NO_PARAMS;

#[derive(Default)]
struct MessageStatuses;

impl DBTable for MessageStatuses {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/message_status/create_table.sql"),
            NO_PARAMS,
        )?;
        Ok(())
    }

    fn drop_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/message_status/drop_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn exists() -> Result<bool, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/message_status/table_exists.sql"))?;
        Ok(stmt.exists(NO_PARAMS)?)
    }

    fn reset() -> Result<(), HErr> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        tx.execute(include_str!("sql/message_status/drop_table.sql"), NO_PARAMS)?;
        tx.execute(
            include_str!("sql/message_status/create_table.sql"),
            NO_PARAMS,
        )?;
        tx.commit()?;
        Ok(())
    }
}
