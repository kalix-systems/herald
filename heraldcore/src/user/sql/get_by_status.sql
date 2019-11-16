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
  status = @1
