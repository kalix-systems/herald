SELECT
  conversation_id,
  title,
  picture,
  color,
  muted,
  pairwise,
  last_active_ts,
  expiration_period,
  status
FROM
  conversations
WHERE
  conversation_id = ?
LIMIT
  1
