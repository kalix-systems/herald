UPDATE OR IGNORE
  conversations
SET
  last_active_ts = @1
WHERE
  conversation_id = @2
