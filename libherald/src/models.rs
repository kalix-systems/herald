use crate::db::Database;
use rusqlite::NO_PARAMS;

pub(crate) mod contact {
    use super::*;
    /// Creates empty contacts table.
    pub fn create_table(db: &mut Database) -> Result<(), rusqlite::Error> {
        db.execute(
            "CREATE TABLE IF NOT EXISTS 'Contacts' (
                    'name' TEXT NOT NULL,
                    PRIMARY KEY(name)
           )",
            NO_PARAMS,
        )?;

        Ok(())
    }

    /// Inserts default contacts into contacts table.
    pub fn insert(db: &mut Database) -> Result<(), rusqlite::Error> {
        let mut stmt = db.prepare("INSERT INTO Contacts VALUES(?)")?;

        stmt.execute(&["Albert Einstein"])?;
        stmt.execute(&["Ernest Hemmingway"])?;
        stmt.execute(&["Hans Gude"])?;

        Ok(())
    }

    pub fn contacts(db: &mut Database) -> Result<Vec<String>, rusqlite::Error> {
        let mut stmt = db.prepare("SELECT * FROM Contacts")?;

        let rows = stmt.query_map(NO_PARAMS, |row| row.get(0))?;

        let mut names: Vec<String> = Vec::new();
        for name_res in rows {
            names.push(name_res?);
        }

        Ok(names)
    }
}

struct Message {
    author: String,
    recipient: String,
    timestamp: String,
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test_derive::serial;

    #[test]
    #[serial]
    fn create_contacts() {
        // start fresh
        crate::utils::delete_db();

        let mut db = Database::new().unwrap();

        contact::create_table(&mut db).unwrap();
    }

    #[test]
    #[serial]
    fn insert_contacts() {
        // start fresh
        crate::utils::delete_db();

        let mut db = Database::new().unwrap();

        contact::create_table(&mut db).unwrap();

        contact::insert(&mut db).unwrap();
    }

    #[test]
    #[serial]
    fn get_contacts() {
        // start fresh
        crate::utils::delete_db();

        let mut db = Database::new().unwrap();

        contact::create_table(&mut db).unwrap();

        contact::insert(&mut db).unwrap();

        let contacts = contact::contacts(&mut db).unwrap();

        assert_eq!(contacts.len(), 3);
    }
}
