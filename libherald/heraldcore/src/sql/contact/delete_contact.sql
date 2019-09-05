UPDATE
  contacts
SET
  name = NULL,
  profile_picture = NULL,
  archived = 0,
  deleted = 1
WHERE
  user_id = ?;