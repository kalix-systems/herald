UPDATE
  conversations
SET
  expiration_period = @expiration_period
WHERE
  conversation_id = @conversation_id
