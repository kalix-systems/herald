UPDATE
  chainkeys
SET
  used = 0
WHERE
  (conversation_id=@1) AND (hash = @2)
