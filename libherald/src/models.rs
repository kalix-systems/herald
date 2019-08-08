use crate::db::Database;
use failure::*;
use rusqlite::NO_PARAMS;

#[allow(dead_code)]
// TODO should this do something?
pub struct Contact {
    name: String,
}

impl Contact {
    /// Creates empty contacts table.
    pub fn create_table(db: &mut Database) -> Result<(), Error> {
        db.execute(
            "CREATE TABLE IF NOT EXISTS 'Contacts' (
                    'name' TEXT NOT NULL,
                    PRIMARY KEY(name)
           )",
            NO_PARAMS,
        )
        .map_err(|e| format_err!("{}", e))?;

        Ok(())
    }

    /// Inserts default contacts into contacts table.
    pub fn insert(db: &mut Database) -> Result<(), Error> {
        db.execute("INSERT INTO Contacts VALUES('Albert Einstein')", NO_PARAMS)
            .map_err(|e| format_err!("{}", e))?;

        db.execute(
            "INSERT INTO Contacts VALUES('Ernest Hemmingway')",
            NO_PARAMS,
        )
        .map_err(|e| format_err!("{}", e))?;

        db.execute("INSERT INTO Contacts VALUES('Hans Gude')", NO_PARAMS)
            .map_err(|e| format_err!("{}", e))?;

        Ok(())
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

        Contact::create_table(&mut db).unwrap();
    }

    #[test]
    #[serial]
    fn insert_contacts() {
        // start fresh
        crate::utils::delete_db();

        let mut db = Database::new().unwrap();

        Contact::create_table(&mut db).unwrap();

        Contact::insert(&mut db).unwrap();
    }
}
