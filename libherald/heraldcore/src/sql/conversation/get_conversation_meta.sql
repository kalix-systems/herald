SELECT
  conversation_id,
  title,
  picture,
  color,
  muted,
  pairwise
FROM
  conversations
WHERE
  conversation_id = ?
LIMIT
  1