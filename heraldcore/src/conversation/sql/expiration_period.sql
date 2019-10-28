SELECT
  expiration_period
FROM
  conversations
WHERE
  conversation_id = @conversation_id
