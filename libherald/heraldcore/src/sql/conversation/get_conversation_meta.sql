SELECT
  conversation_id,
  title,
  picture,
  color,
  muted
FROM
  conversations
WHERE
  conversation_id = ?
LIMIT
  1