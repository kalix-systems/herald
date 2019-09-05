use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use herald_common::MessageStatus;
use rusqlite::{ToSql, NO_PARAMS};
use std::convert::TryInto;
static DATE_FMT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Default)]
/// Messages
pub struct Messages {}

/// Message
pub struct Message {
    /// Local message id
    pub message_id: i64,
    /// Author user id
    pub author: String,
    /// Recipient user id
    pub recipient: String,
    /// Body of message
    pub body: String,
    /// Time the message was sent or received at the server.
    pub timestamp: DateTime<Utc>,
    /// has anyone seen this message yet.
    pub message_status: MessageStatus,
}

impl Message {
    fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let message_id: i64 = row.get(0)?;
        let author: String = row.get(1)?;
        let recipient: String = row.get(2)?;
        let body: String = row.get(3)?;
        let timestamp: String = row.get(4)?;
        let message_status: u8 = row.get(5)?;

        Ok(Message {
            message_id,
            author,
            recipient,
            body,
            timestamp: match NaiveDateTime::parse_from_str(timestamp.as_str(), DATE_FMT) {
                Ok(ts) => DateTime::from_utc(ts, Utc),
                Err(e) => {
                    eprintln!("Error parsing time stamp {}", e);
                    Utc::now()
                }
            },
            message_status: message_status.try_into().unwrap_or(MessageStatus::NoAck),
        })
    }
}

impl Messages {
    /// Adds a message to the database.
    pub fn add_message(
        author: &str,
        recipient: &str,
        body: &str,
        timestamp: Option<DateTime<Utc>>,
        status: MessageStatus,
    ) -> Result<(i64, String), HErr> {
        let timestamp = match timestamp {
            Some(ts) => ts,
            None => Utc::now(),
        }
        .format(DATE_FMT)
        .to_string();

        let body = body.to_sql()?;
        let db = Database::get()?;
        db.execute(
            include_str!("sql/message/add.sql"),
            &[
                author.to_sql()?,
                recipient.to_sql()?,
                body.to_sql()?,
                timestamp.to_sql()?,
                (status as u32).to_sql()?,
            ],
        )?;
        Ok((db.last_insert_rowid(), timestamp))
    }

    /// Deletes a message
    pub fn delete_message(id: i64) -> Result<(), HErr> {
        let db = Database::get()?;

        db.execute(include_str!("sql/message/delete.sql"), &[id])?;
        Ok(())
    }

    /// Get all messages with `user_id` as author or recipient.
    pub fn get_conversation(id: &str) -> Result<Vec<Message>, HErr> {
        let db = Database::get()?;

        let mut stmt = db.prepare(include_str!("sql/message/get_conversation.sql"))?;
        let res = stmt.query_map(&[id.to_sql()?], Message::from_db)?;

        let mut msgs = Vec::new();
        for msg in res {
            msgs.push(msg?);
        }

        Ok(msgs)
    }

    /// sets the message status of an item in the DB
    /// currently assumes conversations are SYNCED
    pub fn update_status(
        conversation_id: &str,
        row: i64,
        status: MessageStatus,
    ) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/message/update_status.sql"),
            &[
                conversation_id.to_sql()?,
                row.to_sql()?,
                (status as u32).to_sql()?,
            ],
        )?;
        Ok(())
    }

    /// Deletes all messages in a conversation.
    pub fn delete_conversation(conversation_id: &str) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/message/delete_conversation.sql"),
            &[conversation_id],
        )?;
        Ok(())
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

        let author = "Hello";
        let recipient = "World";
        Messages::add_message(author, recipient, "1", None, MessageStatus::NoAck)
            .expect("Failed to add first message");
        Messages::add_message(author, recipient, "2", None, MessageStatus::NoAck)
            .expect("Failed to add second message");

        let v1 = Messages::get_conversation(author).expect("Failed to get conversation by author");
        assert_eq!(v1.len(), 2);
        let v2 =
            Messages::get_conversation(recipient).expect("Failed to get conversation by recipient");
        assert_eq!(v2.len(), 2);
    }

    #[test]
    #[serial]
    fn delete_message() {
        Messages::drop_table().unwrap();
        Messages::create_table().unwrap();

        let author = "Hello";
        let recipient = "World";
        Messages::add_message(author, recipient, "1", None, MessageStatus::NoAck)
            .expect("Failed to add first message");

        Messages::delete_message(1).unwrap();

        assert!(Messages::get_conversation(author).unwrap().is_empty());
    }

    #[test]
    #[serial]
    fn delete_conversation() {
        Messages::drop_table().unwrap();
        Messages::create_table().unwrap();

        let author = "Hello";
        let recipient = "World";
        Messages::add_message(author, recipient, "1", None, MessageStatus::NoAck)
            .expect("Failed to add first message");
        Messages::add_message(recipient, author, "1", None, MessageStatus::NoAck)
            .expect("Failed to add first message");

        Messages::delete_conversation(recipient).unwrap();

        assert!(Messages::get_conversation(author).unwrap().is_empty());
    }

    #[test]
    #[serial]
    fn message_status_updates() {
        Messages::drop_table().unwrap();
        Messages::create_table().unwrap();

        let author = "Hello";
        let recipient = "World";
        Messages::add_message(author, recipient, "1", None, MessageStatus::NoAck)
            .expect("Failed to add first message");

        assert_eq!(
            Messages::get_conversation(author).expect("failed to get conversation by author")[0]
                .message_status,
            MessageStatus::NoAck
        );

        let (row, _) = Messages::add_message(
            author,
            recipient,
            "new",
            None,
            MessageStatus::RecipReceivedAck,
        )
        .expect("Failed to add first message");

        assert_eq!(
            Messages::get_conversation(author).expect("failed to get conversation by author")[1]
                .message_status,
            MessageStatus::RecipReceivedAck
        );

        Messages::update_status(recipient, row, MessageStatus::Timeout)
            .expect("could not update status :");
        //if this fails the UPDATE call was not specific enough
        assert_eq!(
            Messages::get_conversation(author)
                .expect("failed to get conversation by author, the second time")[0]
                .message_status,
            MessageStatus::NoAck
        );

        assert_eq!(
            Messages::get_conversation(author)
                .expect("failed to get conversation by author, the third time")[1]
                .message_status,
            MessageStatus::Timeout
        );
    }
}
