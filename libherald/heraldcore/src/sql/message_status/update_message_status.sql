UPDATE
  message_status
SET
  status = @1
WHERE
  (msg_id, user_id) = (@2, @3)