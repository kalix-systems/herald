SELECT
  msg_id,
  author,
  conversation,
  body,
  op,
  timestamp
FROM
  messages
WHERE
  conversation = ?
ORDER BY
  timestamp ASC