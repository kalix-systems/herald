SELECT
  author,
  body,
  op_msg_id,
  insertion_ts,
  server_ts,
  expiration_ts,
  send_status,
  is_reply,
  messages.aux_item
FROM
  messages LEFT OUTER JOIN replies ON messages.msg_id = replies.msg_id
WHERE
  messages.msg_id = @msg_id
LIMIT
  1
