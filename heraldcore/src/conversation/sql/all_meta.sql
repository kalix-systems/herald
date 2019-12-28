SELECT
  conversations.conversation_id,
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
ORDER BY
  last_active_ts DESC
