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
        db.execute(
            "CREATE TABLE IF NOT EXISTS conversations (
               author TEXT NOT NULL,
               recipient TEXT NOT NULL,
               timestamp TEXT NOT NULL,
               message TEXT NOT NULL,
               FOREIGN KEY(author) REFERENCES contacts (name),
               FOREIGN KEY(recipient) REFERENCES contacts (name)
            )",
            NO_PARAMS,
        )?;

        Ok(())
    }

    fn drop_table(&mut self) -> Result<(), HErr> {
        let db = &self.db;
        db.execute("DROP TABLE IF EXISTS conversations", NO_PARAMS)?;
        Ok(())
    }

    fn exists(&self) -> bool {
        let db = &self.db;

        let cnt = db
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='conversations'",
                NO_PARAMS,
                |row| row.get(0),
            )
            .unwrap_or(0);

        cnt > 0
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
