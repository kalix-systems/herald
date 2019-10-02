SELECT
  pairwise_conversation
FROM
  contacts
WHERE
  user_id = ?
LIMIT
  1
