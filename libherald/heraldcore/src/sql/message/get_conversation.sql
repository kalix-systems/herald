SELECT
  author,
  recipient,
  body,
  timestamp
FROM
  messages
WHERE
  author = @1
  OR recipient = @1
ORDER BY
  timestamp DESC