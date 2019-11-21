SELECT
  channel_key
FROM
  channel_keys
WHERE
  (conversation_id = @1)
LIMIT
  1
