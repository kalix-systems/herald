SELECT
  messages.msg_id,
  author,
  conversation_id,
  body,
  op_msg_id,
  ts,
  receipts,
  send_status,
  has_attachments
FROM
  messages LEFT OUTER JOIN replies ON messages.msg_id = replies.msg_id
WHERE
  send_status = ?
ORDER BY
  ts DESC
