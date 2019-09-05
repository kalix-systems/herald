use crate::{
    db::{DBTable, Database},
    errors::HErr,
    utils::ConversationId,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::{ToSql, NO_PARAMS};

static DATE_FMT: &str = "%Y-%m-%d %H:%M:%S";

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
    pub message_id: i64,
    /// Author user id
    pub author: String,
    /// Recipient user id
    pub conversation: ConversationId,
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
    fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let message_id: i64 = row.get(0)?;
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
            timestamp: match NaiveDateTime::parse_from_str(timestamp.as_str(), DATE_FMT) {
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
        author: &str,
        conversation: &ConversationId,
        body: &str,
        timestamp: Option<DateTime<Utc>>,
        op: Option<i64>,
        status: MessageStatus,
    ) -> Result<(i64, DateTime<Utc>), HErr> {
        let timestamp = match timestamp {
            Some(ts) => ts,
            None => Utc::now(),
        };

        let timestamp_string = timestamp.format(DATE_FMT).to_string();

        let body = body.to_sql()?;

        let db = Database::get()?;

        db.execute(
            include_str!("sql/message/add.sql"),
            &[
                author.to_sql()?,
                conversation.to_sql()?,
                body.to_sql()?,
                timestamp_string.to_sql()?,
                op.to_sql()?,
                (status as u32).to_sql()?,
            ],
        )?;
        Ok((db.last_insert_rowid(), timestamp))
    }

    /// Deletes a message
    pub fn delete_message(id: i64) -> Result<(), HErr> {
        let db = Database::get()?;

        db.execute(include_str!("sql/message/delete_message.sql"), &[id])?;
        Ok(())
    }

    /// Get all messages with `user_id` as author or recipient.
    pub fn get_conversation(id: &ConversationId) -> Result<Vec<Message>, HErr> {
        let db = Database::get()?;

        let mut stmt = db.prepare(include_str!("sql/message/get_conversation.sql"))?;
        let res = stmt.query_map(&[id.to_sql()?], Message::from_db)?;

        let mut msgs = Vec::new();
        for msg in res {
            msgs.push(msg?);
        }

        Ok(msgs)
    }

    /// Deletes all messages in a conversation.
    pub fn delete_conversation(conversation_id: &ConversationId) -> Result<(), HErr> {
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
    use crate::{contact::Contacts, conversation::Conversations, db::Database};
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

    #[test]
    #[serial]
    fn add_and_get() {
        Database::reset_all().expect(womp!());

        let author = "Hello";
        Contacts::add(author, None, None, None).expect(womp!());

        let conversation = vec![0; 32];
        Conversations::add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

        Messages::add_message(author, &conversation, "1", None, None, MessageStatus::NoAck)
            .expect(womp!("Failed to add first message"));

        Messages::add_message(author, &conversation, "2", None, None, MessageStatus::NoAck)
            .expect(womp!("Failed to add second message"));

        let v1 = Messages::get_conversation(&conversation)
            .expect(womp!("Failed to get conversation by author"));
        assert_eq!(v1.len(), 2);
        let v2 = Messages::get_conversation(&conversation)
            .expect(womp!("Failed to get conversation by recipient"));
        assert_eq!(v2.len(), 2);
    }

    #[test]
    #[serial]
    fn delete_message() {
        Database::reset_all().expect(womp!());

        let author = "Hello";
        Contacts::add(author, None, None, None).expect(womp!());

        let conversation = vec![0; 32];
        Conversations::add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

        Messages::add_message(author, &conversation, "1", None, None, MessageStatus::NoAck)
            .expect(womp!("Failed to add first message"));

        Messages::delete_message(1).expect(womp!());

        assert!(Messages::get_conversation(&conversation)
            .expect(womp!())
            .is_empty());
    }

    #[test]
    #[serial]
    fn delete_conversation() {
        Database::reset_all().expect(womp!());

        let author = "Hello";
        Contacts::add(author, None, None, None).expect(womp!());

        let conversation = vec![0; 32];
        Conversations::add_conversation(Some(&conversation), None)
            .expect(womp!("Failed to create conversation"));

        let author = "Hello";
        let conversation = vec![0; 32];
        Messages::add_message(author, &conversation, "1", None, None, MessageStatus::NoAck)
            .expect(womp!("Failed to add first message"));
        Messages::add_message(author, &conversation, "1", None, None, MessageStatus::NoAck)
            .expect(womp!("Failed to add first message"));

        Messages::delete_conversation(&conversation).expect(womp!());

        assert!(Messages::get_conversation(&conversation)
            .expect(womp!())
            .is_empty());
    }
}
