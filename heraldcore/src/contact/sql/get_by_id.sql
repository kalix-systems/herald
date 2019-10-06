SELECT
  user_id,
  name,
  profile_picture,
  color,
  status,
  pairwise_conversation,
  contact_type,
  added
FROM
  contacts
WHERE
  user_id = ?
LIMIT
  1