use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use chrono::{DateTime, Utc};
use herald_common::{RawMsg, UserId};
use rusqlite::{ToSql, NO_PARAMS};

#[derive(Default)]
pub struct Messages {}

pub struct Message {
    author: UserId,
    recipient: UserId,
    body: RawMsg,
    timestamp: Option<DateTime<Utc>>,
}

impl Messages {
    pub fn add_message(
        author: UserId,
        recipient: UserId,
        body: RawMsg,
        timestamp: Option<DateTime<Utc>>,
    ) -> Result<(), HErr> {
        let timestamp = match timestamp {
            Some(ts) => ts,
            None => Utc::now(),
        }
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
        let recipient = recipient.to_string();
        let author = author.to_string();
        let body = body.to_sql()?;

        let db = Database::get()?;

        db.execute(
            include_str!("sql/message/add.sql"),
            &[
                author.to_sql()?,
                recipient.to_sql()?,
                body.to_sql()?,
                timestamp.to_sql()?,
            ],
        )?;
        Ok(())
    }

    /// Get all messages with `id`
    pub fn get_conversation(id: UserId) -> Result<Vec<Message>, HErr> {
        let id = id.to_string();
        let db = Database::get()?;

        let mut stmt = db.prepare(include_str!("sql/message/get_conversation.sql"))?;
        let res = stmt.query_map(&[id.to_sql()?], |row| {
            let author: String = row.get(0)?;
            let recipient: String = row.get(1)?;
            let body: Vec<u8> = row.get(2)?;
            let timestamp: String = row.get(3)?;
            Ok(Message {
                author: UserId::from(&author).unwrap(),
                recipient: UserId::from(&recipient).unwrap(),
                body: RawMsg::from(body),
                timestamp: timestamp.parse().ok(),
            })
        })?;

        let mut msgs = Vec::new();
        for msg in res {
            msgs.push(msg?);
        }

        Ok(msgs)
    }
}

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

    #[test]
    #[serial]
    fn add_and_get() {
        Messages::drop_table().unwrap();
        Messages::create_table().unwrap();

        let author = UserId::from("Hello").unwrap();
        let recipient = UserId::from("World").unwrap();
        Messages::add_message(author, recipient, RawMsg::from("1"), None).unwrap();
        Messages::add_message(author, recipient, RawMsg::from("2"), None).unwrap();

        let v1 = Messages::get_conversation(author).unwrap();
        assert_eq!(v1.len(), 2);
        let v2 = Messages::get_conversation(recipient).unwrap();
        assert_eq!(v2.len(), 2);
    }
}
