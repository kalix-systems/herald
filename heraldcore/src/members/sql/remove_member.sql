DELETE FROM
  conversation_members
WHERE
  conversation_id = @1
  AND member_id = @2