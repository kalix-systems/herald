SELECT
  user_id,
  name,
  profile_picture,
  color,
  status,
  pairwise_conversation
FROM
  contacts
WHERE
  status = ?