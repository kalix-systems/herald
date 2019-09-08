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
  msg_id = ?
LIMIT
  1