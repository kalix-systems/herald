use super::*;
use coremacros::w;
use rusqlite::named_params;

pub(crate) fn add_members_with_tx(
    tx: &rusqlite::Transaction,
    cid: ConversationId,
    members: &[UserId],
) -> Result<(), HErr> {
    let mut stmt = w!(tx.prepare(include_str!("sql/add_member.sql")));

    for member_id in members {
        w!(stmt.execute(params![&cid, member_id]));
    }

    Ok(())
}

/// Add a user with `member_id` to the conversation with `conversation_id`.
pub(crate) fn add_member(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    member_id: UserId,
) -> Result<(), HErr> {
    w!(conn.execute(
        include_str!("sql/add_member.sql"),
        params![conversation_id, member_id],
    ));
    Ok(())
}

/// Remove a user with `member_id` to the conversation with `conversation_id`.
pub(crate) fn remove_member(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
    member_id: UserId,
) -> Result<(), HErr> {
    w!(conn.execute(
        include_str!("sql/remove_member.sql"),
        params![conversation_id, member_id],
    ));

    Ok(())
}

/// Gets the members of a conversation.
pub(crate) fn members(
    conn: &rusqlite::Connection,
    conversation_id: &ConversationId,
) -> Result<Vec<UserId>, HErr> {
    let mut stmt = w!(conn.prepare(include_str!("sql/get_conversation_members.sql")));
    let res = w!(stmt.query_map(params![conversation_id], |row| row.get(0)));

    let mut members = Vec::new();
    for member in res {
        members.push(member?);
    }

    Ok(members)
}

/// Gets the conversations shared with a user
pub(crate) fn shared_conversations(
    conn: &rusqlite::Connection,
    user_id: &UserId,
) -> Result<Vec<ConversationId>, rusqlite::Error> {
    let mut stmt = w!(conn.prepare_cached(include_str!("sql/shared_conversations.sql")));

    let mut shared = Vec::new();
    let res = w!(
        stmt.query_map_named(named_params! { "@user_id": user_id }, |row| row
            .get("conversation_id"))
    );

    for cid in res {
        shared.push(cid?);
    }

    Ok(shared)
}
