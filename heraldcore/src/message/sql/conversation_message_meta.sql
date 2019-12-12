SELECT
  messages.msg_id,
  messages.insertion_ts
FROM
  messages
WHERE
  messages.conversation_id = @conversation_id
ORDER BY
  insertion_ts ASC
