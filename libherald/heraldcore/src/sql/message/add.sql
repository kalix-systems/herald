INSERT INTO
  messages(
    msg_id,
    author,
    conversation_id,
    body,
    timestamp,
    op_msg_id,
    send_stats
  )
VALUES(@1, @2, @3, @4, @5, @6, @7)