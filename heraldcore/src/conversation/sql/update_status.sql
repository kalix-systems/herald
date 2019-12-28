UPDATE
  conversations
SET
  status = @1
WHERE
  conversation_id = @2
