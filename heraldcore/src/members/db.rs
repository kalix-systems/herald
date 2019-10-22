use super::*;

pub(crate) fn add_members_with_tx(
    tx: &rusqlite::Transaction,
    cid: ConversationId,
    members: &[UserId],
) -> Result<(), HErr> {
    let mut stmt = tx.prepare(include_str!("sql/add_member.sql"))?;

    for member_id in members {
        stmt.execute(params![&cid, member_id])?;
    }

    Ok(())
}

/// Add a user with `member_id` to the conversation with `conversation_id`.
pub fn add_member_db(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    member_id: UserId,
) -> Result<(), HErr> {
    conn.execute(
        include_str!("sql/add_member.sql"),
        params![conversation_id, member_id],
    )?;
    Ok(())
}

/// Remove a user with `member_id` to the conversation with `conversation_id`.
pub fn remove_member_db(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    member_id: UserId,
) -> Result<(), HErr> {
    conn.execute(
        include_str!("sql/remove_member.sql"),
        params![conversation_id, member_id],
    )?;
    Ok(())
}

/// Gets the members of a conversation.
pub fn members_db(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<Vec<UserId>, HErr> {
    let mut stmt = conn.prepare(include_str!("sql/get_conversation_members.sql"))?;
    let res = stmt.query_map(params![conversation_id], |row| row.get(0))?;

    let mut members = Vec::new();
    for member in res {
        members.push(member?);
    }

    Ok(members)
}
