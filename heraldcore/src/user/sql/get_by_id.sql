SELECT
  user_id,
  name,
  profile_picture,
  color,
  status,
  pairwise_conversation,
  user_type
FROM
  users
WHERE
  user_id = ?
LIMIT
  1
