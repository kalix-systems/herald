SELECT
  msg_id,
  author,
  conversation_id,
  body,
  op_msg_id,
  timestamp
FROM
  messages
WHERE
  conversation_id = ?
ORDER BY
  timestamp ASC