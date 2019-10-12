SELECT
  conversations.conversation_id,
  title,
  picture,
  color,
  muted,
  pairwise
FROM
  conversations
LEFT OUTER JOIN
  messages
ON
  messages.conversation_id = conversations.conversation_id
GROUP BY
  conversations.conversation_id
ORDER BY
  messages.ts DESC
