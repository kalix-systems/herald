SELECT
  msg_id,
  author,
  conversation_id,
  body,
  op_msg_id,
  timestamp,
  send_status
FROM
  messages
WHERE
  conversation_id = @1
  AND timestamp > @2
ORDER BY
  timestamp ASC