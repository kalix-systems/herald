UPDATE
  messages
SET
  receipts = @1
WHERE
  msg_id = @2
