use crate::{
    db::{DBTable, Database},
    errors::HErr,
    utils,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::{params, NO_PARAMS};

/// the network status of a message
#[derive(Clone)]
pub enum MessageStatus {
    /// No ack from any third party
    NoAck = 0,
    /// Received by the server
    ServerAck = 1,
    /// Received by the recipient
    RecipientAck = 2,
    /// The message has timedout.
    Timeout = 3,
    /// we did not write this message
    Inbound = 4,
}

#[derive(Default, Clone)]
/// Messages
pub struct Messages {}

/// Message
#[derive(Clone)]
pub struct Message {
    /// Local message id
    pub message_id: Vec<u8>,
    /// Author user id
    pub author: String,
    /// Recipient user id
    pub conversation: Vec<u8>,
    /// Body of message
    pub body: String,
    /// Time the message was sent or received at the server.
    pub timestamp: DateTime<Utc>,
    /// Message id of the message being replied to
    pub op: Option<i64>,
    /// has anyone seen this message yet.
    pub message_status: MessageStatus,
}

impl Message {
    pub(crate) fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let message_id: Vec<u8> = row.get(0)?;
        let author: String = row.get(1)?;
        let conversation: Vec<u8> = row.get(2)?;
        let body: String = row.get(3)?;
        let op: Option<i64> = row.get(4)?;
        let timestamp: String = row.get(5)?;

        Ok(Message {
            message_id,
            author,
            conversation,
            body,
            op,
            timestamp: match NaiveDateTime::parse_from_str(timestamp.as_str(), utils::DATE_FMT) {
                Ok(ts) => DateTime::from_utc(ts, Utc),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    Utc::now()
                }
            },
            message_status: MessageStatus::NoAck,
        })
    }
}

impl Messages {
    /// Adds a message to the database.
    pub fn add_message(
        msg_id: Option<Vec<u8>>,
        author: &str,
        conversation: &[u8],
        body: &str,
        timestamp: Option<DateTime<Utc>>,
        op: Option<i64>,
    ) -> Result<(Vec<u8>, DateTime<Utc>), HErr> {
        let timestamp = timestamp.unwrap_or_else(Utc::now);
        let timestamp_string = timestamp.format(utils::DATE_FMT).to_string();

        let db = Database::get()?;

        let msg_id = msg_id.unwrap_or_else(|| utils::rand_id().to_vec());

        db.execute(
            include_str!("sql/message/add.sql"),
            params![msg_id, author, conversation, body, timestamp_string, op,],
        )?;
        Ok((msg_id, timestamp))
    }

    /// Deletes a message
    pub fn delete_message(id: &[u8]) -> Result<(), HErr> {
        let db = Database::get()?;

        db.execute(include_str!("sql/message/delete_message.sql"), &[id])?;
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

    fn reset() -> Result<(), HErr> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        tx.execute(include_str!("sql/message/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/message/create_table.sql"), NO_PARAMS)?;
        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test_derive::serial;

    use womp::*;

    #[test]
    #[serial]
    fn create_drop_exists() {
        // drop twice, it shouldn't panic on multiple drops
        Messages::drop_table().expect(womp!());
        Messages::drop_table().expect(womp!());

        Messages::create_table().expect(womp!());
        assert!(Messages::exists().expect(womp!()));
        Messages::create_table().expect(womp!());
        assert!(Messages::exists().expect(womp!()));
        Messages::drop_table().expect(womp!());
        assert!(!Messages::exists().expect(womp!()));
    }
}
