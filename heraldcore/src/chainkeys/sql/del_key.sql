DELETE FROM
  chainkeys
WHERE
  (conversation_id = @1) AND (hash = @2)
