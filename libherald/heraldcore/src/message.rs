use crate::{
    db::{DBTable, Database},
    errors::HErr,
    types::*,
    utils,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use herald_common::*;
use rusqlite::{params, NO_PARAMS};

#[derive(Default, Clone)]
/// Messages
pub struct Messages {
    db: Database,
}

/// Message
#[derive(Clone)]
pub struct Message {
    /// Local message id
    pub message_id: MsgId,
    /// Author user id
    pub author: UserId,
    /// Recipient user id
    pub conversation: ConversationId,
    /// Body of message
    pub body: String,
    /// Time the message was sent (if outbound) or received at the server (if inbound).
    pub timestamp: DateTime<Utc>,
    /// Message id of the message being replied to
    pub op: Option<MsgId>,
    /// Send status
    pub send_status: Option<MessageSendStatus>,
    /// Receipts
    pub receipts: Option<Vec<(UserId, MessageReceiptStatus)>>,
}

impl Message {
    pub(crate) fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let timestamp =
            match NaiveDateTime::parse_from_str(row.get::<_, String>(5)?.as_str(), utils::DATE_FMT)
            {
                Ok(ts) => DateTime::from_utc(ts, Utc),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    Utc::now()
                }
            };

        Ok(Message {
            message_id: row.get(0)?,
            author: row.get(1)?,
            conversation: row.get(2)?,
            body: row.get(3)?,
            op: row.get(4)?,
            timestamp,
            receipts: None,
            send_status: row.get(6)?,
        })
    }
}

/// Adds a message to the database.
fn add_message(
    db: &Database,
    msg_id: Option<MsgId>,
    author: UserIdRef,
    conversation_id: &ConversationId,
    body: &str,
    timestamp: Option<DateTime<Utc>>,
    op: &Option<MsgId>,
) -> Result<(MsgId, DateTime<Utc>), HErr> {
    let timestamp = timestamp.unwrap_or_else(Utc::now);
    let timestamp_param = timestamp.format(utils::DATE_FMT).to_string();

    let msg_id = msg_id.unwrap_or_else(|| utils::rand_id().into());
    db.execute(
        include_str!("sql/message/add.sql"),
        params![msg_id, author, conversation_id, body, timestamp_param, op,],
    )?;
    Ok((msg_id, timestamp))
}
/// Get message by message id
fn get_message(db: &Database, msg_id: &MsgId) -> Result<Message, HErr> {
    Ok(db.query_row(
        include_str!("sql/message/get_message.sql"),
        params![msg_id],
        Message::from_db,
    )?)
}
/// Sets the message status of an item in the database
fn update_send_status(db: &Database, msg_id: MsgId, status: MessageSendStatus) -> Result<(), HErr> {
    db.execute(
        include_str!("sql/message/update_send_status.sql"),
        params![status, msg_id],
    )?;
    Ok(())
}

/// Deletes a message
fn delete_message(db: &Database, id: &MsgId) -> Result<(), HErr> {
    db.execute(include_str!("sql/message/delete_message.sql"), params![id])?;
    Ok(())
}

impl Messages {
    /// Create new `Messages`
    pub fn new() -> Result<Self, HErr> {
        Ok(Self {
            db: Database::get()?,
        })
    }

    /// Adds a message to the database.
    pub fn add_message(
        &self,
        msg_id: Option<MsgId>,
        author: UserIdRef,
        conversation_id: &ConversationId,
        body: &str,
        timestamp: Option<DateTime<Utc>>,
        op: &Option<MsgId>,
    ) -> Result<(MsgId, DateTime<Utc>), HErr> {
        add_message(
            &self.db,
            msg_id,
            author,
            conversation_id,
            body,
            timestamp,
            op,
        )
    }

    /// Get message by message id
    pub fn get_message(&self, msg_id: &MsgId) -> Result<Message, HErr> {
        get_message(&self.db, msg_id)
    }

    /// Sets the message status of an item in the database
    pub fn update_send_status(&self, msg_id: MsgId, status: MessageSendStatus) -> Result<(), HErr> {
        update_send_status(&self.db, msg_id, status)
    }

    /// Deletes a message
    pub fn delete_message(&self, id: &MsgId) -> Result<(), HErr> {
        delete_message(&self.db, id)
    }
}

impl DBTable for Messages {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/message_status/create_table.sql"),
            NO_PARAMS,
        )?;
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
    use crate::conversation::Conversations;
    use serial_test_derive::serial;

    use womp::*;

    #[test]
    #[serial]
    fn create_drop_exists() {
        Database::reset_all().expect(womp!());
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
    fn message_send_status_updates() {
        Database::reset_all().expect(womp!());

        let conversation_id = [0; 32].into();
        crate::conversation::Conversations::add_conversation(Some(&conversation_id), None)
            .expect(womp!());

        let author = "Hello";
        crate::contact::ContactBuilder::new(author.into())
            .add()
            .expect(womp!());
        crate::members::Members::add_member(&conversation_id, author).expect(womp!());

        let handle = Messages::new().expect(womp!());

        let (msg_id, _) = handle
            .add_message(None, author, &conversation_id, "1", None, &None)
            .expect(womp!("Failed to add first message"));

        assert_eq!(
            handle
                .get_message(&msg_id)
                .expect(womp!("failed to get conversation by author"))
                .send_status,
            None,
        );

        handle
            .update_send_status(msg_id, MessageSendStatus::Ack)
            .expect(womp!());

        assert_eq!(
            Conversations::get_conversation_messages(&conversation_id)
                .expect(womp!("failed to get conversation by author"))[0]
                .send_status,
            Some(MessageSendStatus::Ack)
        );
    }
}
