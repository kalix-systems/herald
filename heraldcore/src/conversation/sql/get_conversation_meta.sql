SELECT
  conversation_id,
  title,
  picture,
  color,
  muted,
  pairwise,
  last_active_ts
FROM
  conversations
WHERE
  conversation_id = ?
LIMIT
  1
