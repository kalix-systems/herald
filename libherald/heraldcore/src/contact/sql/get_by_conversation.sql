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
  user_id IN (
    SELECT
      member_id
    FROM
      conversation_members
    WHERE
      conversation_id = @1
  )
  AND added > @2