SELECT
  pairwise_conversation
FROM
  users
WHERE
  user_id = ?
LIMIT
  1
