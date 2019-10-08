SELECT
  messages.msg_id,
  author,
  conversation_id,
  body,
  op_msg_id,
  ts,
  receipts,
  send_status
FROM
  messages LEFT OUTER JOIN replies ON messages.msg_id = replies.msg_id
WHERE
  messages.msg_id = ?
LIMIT
  1
