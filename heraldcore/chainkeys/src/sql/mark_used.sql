UPDATE
  chainkeys
SET
  used = 1
WHERE
  (conversation_id=@1) AND (hash = @2)
