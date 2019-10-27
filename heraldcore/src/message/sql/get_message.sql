SELECT
  messages.msg_id,
  author,
  conversation_id,
  body,
  op_msg_id,
  insertion_ts,
  server_ts,
  expiration_ts,
  send_status,
  has_attachments
FROM
  messages LEFT OUTER JOIN replies ON messages.msg_id = replies.msg_id
WHERE
  messages.msg_id = @msg_id AND messages.known = 1
LIMIT
  1
