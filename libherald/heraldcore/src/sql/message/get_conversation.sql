SELECT
  msg_id,
  author,
  recipient,
  body,
  op,
  timestamp
FROM
  messages
WHERE
  author = @1
  OR recipient = @1
ORDER BY
  timestamp ASC