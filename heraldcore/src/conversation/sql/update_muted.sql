UPDATE
  conversations
SET
  muted = @muted
WHERE
  conversation_id = @conversation_id
