SELECT
  messages.msg_id,
  messages.insertion_ts
FROM
  messages
WHERE
  messages.conversation_id = @conversation_id AND
  (messages.expiration_ts IS NULL OR messages.expiration_ts > @current_time)
ORDER BY
  insertion_ts ASC
