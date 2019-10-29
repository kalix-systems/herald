SELECT DISTINCT
  conversation_id
FROM
  messages
WHERE
  messages.expiration_ts < @time
