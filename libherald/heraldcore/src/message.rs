use crate::{
    db::{DBTable, Database},
    errors::HErr,
    utils,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use herald_common::{
    ConversationId, MessageReceiptStatus, MessageSendStatus, MsgId, UserId, UserIdRef,
};
use rusqlite::{params, NO_PARAMS};

#[derive(Default, Clone)]
/// Messages
pub struct Messages {}

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
    /// Time the message was sent or received at the server.
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
        let message_id: Vec<u8> = row.get(0)?;
        let author: UserId = row.get(1)?;
        let conversation_id: Vec<u8> = row.get(2)?;
        let body: String = row.get(3)?;
        let op: Option<Vec<u8>> = row.get(4)?;
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
            message_id: message_id.into_iter().collect(),
            author,
            conversation: conversation_id.into_iter().collect(),
            body,
            op: op.map(|op| op.into_iter().collect()),
            timestamp,
            receipts: None,
            send_status: None,
        })
    }
}

impl Messages {
    /// Adds a message to the database.
    pub fn add_message(
        msg_id: Option<MsgId>,
        author: UserIdRef,
        conversation_id: &ConversationId,
        body: &str,
        timestamp: Option<DateTime<Utc>>,
        op: Option<MsgId>,
    ) -> Result<(MsgId, DateTime<Utc>), HErr> {
        let timestamp = timestamp.unwrap_or_else(Utc::now);
        let timestamp_param = timestamp.format(utils::DATE_FMT).to_string();

        let msg_id = msg_id.unwrap_or_else(|| utils::rand_id().into());

        let db = Database::get()?;
        db.execute(
            include_str!("sql/message/add.sql"),
            params![
                msg_id.as_slice(),
                author,
                conversation_id.as_slice(),
                body,
                timestamp_param,
                op.map(|x| x.to_vec()),
            ],
        )?;
        Ok((msg_id, timestamp))
    }

    /// Sets the message status of an item in the database
    pub fn update_send_status(msg_id: MsgId, status: MessageSendStatus) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("sql/message/update_status.sql"),
            params![status as u8, msg_id.as_slice()],
        )?;
        Ok(())
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
    // use crate::conversation::Conversations;
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

    //#[test]
    //#[serial]
    //fn message_status_updates() {
    //    Messages::reset().expect(womp!());

    //    let author = "Hello";
    //    let conversation_id = [0; 32];
    //    Messages::add_message(
    //        None,
    //        author,
    //        &conversation_id,
    //        "1",
    //        None,
    //        None,
    //    )
    //    .expect(womp!("Failed to add first message"));

    //    assert_eq!(
    //        Conversations::get_conversation(&conversation_id)
    //            .expect(womp!("failed to get conversation by author"))[0]
    //            .message_status,
    //        MessageStatus::NoAck
    //    );

    //    let (msg_id, _) = Messages::add_message(
    //        None,
    //        author,
    //        &conversation_id,
    //        "new",
    //        None,
    //        None,
    //        MessageStatus::RecipReceivedAck,
    //    )
    //    .expect(womp!("Failed to add first message"));

    //    assert_eq!(
    //        Conversations::get_conversation(&conversation_id)
    //            .expect(womp!("failed to get conversation by author"))[1]
    //            .message_status,
    //        MessageStatus::RecipReceivedAck
    //    );

    //    Messages::update_status(&conversation_id, &msg_id, MessageStatus::Timeout)
    //        .expect(womp!("could not update status :"));
    //    //if this fails the UPDATE call was not specific enough
    //    assert_eq!(
    //        Conversations::get_conversation(&conversation_id).expect(womp!(
    //            "failed to get conversation by author, the second time"
    //        ))[0]
    //            .message_status,
    //        MessageStatus::NoAck
    //    );

    //    assert_eq!(
    //        Conversations::get_conversation(&conversation_id)
    //            .expect("failed to get conversation by author, the third time")[1]
    //            .message_status,
    //        MessageStatus::Timeout
    //    );
    //}
}
