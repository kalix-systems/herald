use super::*;

pub(crate) fn get_receipts(
    conn: &rusqlite::Connection,
    msg_id: &MsgId,
) -> Result<HashMap<UserId, MessageReceiptStatus>, rusqlite::Error> {
    let mut get_stmt = w!(conn.prepare_cached(include_str!("../sql/get_receipts.sql")));

    let res = w!(get_stmt.query_map(params![msg_id], |row| Ok((row.get(0)?, row.get(1)?))));
    Ok(w!(res.collect()))
}

pub(crate) fn add_receipt(
    conn: &Conn,
    msg_id: MsgId,
    recip: UserId,
    receipt_status: MessageReceiptStatus,
) -> Result<(), rusqlite::Error> {
    let mut stmt = w!(conn.prepare_cached(include_str!("../sql/add_receipt.sql")));
    drop(stmt.execute(params![msg_id, recip, receipt_status]));
    Ok(())
}
