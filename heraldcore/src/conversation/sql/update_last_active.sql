UPDATE OR IGNORE
  conversations
SET
  last_active_ts = @1,
  status = @2
WHERE
  conversation_id = @3
