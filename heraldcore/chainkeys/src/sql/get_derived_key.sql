SELECT
  msg_key
FROM
  derived_keys
WHERE
  conversation_id = @1
  AND ix = @2

