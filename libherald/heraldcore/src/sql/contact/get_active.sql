SELECT
  user_id,
  name,
  profile_picture,
  color,
  status
FROM
  contacts
WHERE
  --- Contact not yet deleted
  status < 2