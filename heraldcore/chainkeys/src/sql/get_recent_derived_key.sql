SELECT
  msg_key
FROM
  derived_keys
WHERE
  conversation_id = @cid
  AND public_key = @pk
  AND ix = @ix
SORT BY
  generation DESC
LIMIT
  1

