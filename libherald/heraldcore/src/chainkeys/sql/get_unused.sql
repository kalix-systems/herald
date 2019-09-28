SELECT
  hash,
  chainkey
FROM
  chainkeys
WHERE
  (conversation_id = @1) AND (used = 0)
