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
  status = @1
  AND added > @2