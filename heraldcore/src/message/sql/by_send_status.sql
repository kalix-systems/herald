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
  messages.update_item,
  is_reply
FROM
  messages LEFT OUTER JOIN replies ON messages.msg_id = replies.msg_id
WHERE
  send_status = @send_status
ORDER BY
  insertion_ts DESC
