SELECT
  chainkey
FROM
  chainkeys
WHERE
  (conversation_id = @1) AND (hash = @2)
LIMIT
  1
