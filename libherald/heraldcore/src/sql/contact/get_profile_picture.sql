SELECT
  profile_picture
FROM
  contacts
WHERE
  user_id = ?
LIMIT
  1