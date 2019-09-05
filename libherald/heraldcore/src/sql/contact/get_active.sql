SELECT
  user_id,
  name,
  profile_picture,
  color,
  archived
FROM
  contacts
WHERE
  archived = 0