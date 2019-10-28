SELECT
  messages.msg_id,
  author,
  body,
  op_msg_id,
  insertion_ts,
  server_ts,
  expiration_ts,
  send_status,
  has_attachments,
  is_reply
FROM
  messages LEFT OUTER JOIN replies ON messages.msg_id = replies.msg_id
WHERE
  messages.conversation_id = @conversation_id
ORDER BY
  insertion_ts ASC
