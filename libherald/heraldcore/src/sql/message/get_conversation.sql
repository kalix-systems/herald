SELECT
  id,
  author,
  recipient,
  body,
  timestamp,
  send_status
FROM
  messages
WHERE
  author = @1
  OR recipient = @1
ORDER BY
  timestamp ASC