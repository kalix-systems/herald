UPDATE
  conversations
SET
  muted = @1
WHERE
  conversation_id = @2