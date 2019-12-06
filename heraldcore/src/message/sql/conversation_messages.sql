SELECT
  messages.msg_id,
  messages.author,
  messages.body,
  replies.op_msg_id,
  messages.insertion_ts,
  messages.server_ts,
  messages.expiration_ts,
  messages.send_status,
  messages.is_reply
FROM
  messages LEFT OUTER JOIN replies ON messages.msg_id = replies.msg_id
WHERE
  messages.conversation_id = @conversation_id
ORDER BY
  insertion_ts ASC
