REPLACE INTO
  messages(
    msg_id,
    author,
    conversation_id,
    body,
    send_status,
    receipts,
    has_attachments,
    ts,
    known
  )
VALUES(@1, @2, @3, @4, @5, @6, @7, @8, 1)
