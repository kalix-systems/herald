SELECT
  conversation_id, msg_id
FROM
  messages
WHERE
  messages.expiration_ts < @time
ORDER BY
  conversation_id
