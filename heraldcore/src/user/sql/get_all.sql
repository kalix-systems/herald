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
  status < 2
ORDER BY
  name COLLATE NOCASE ASC
