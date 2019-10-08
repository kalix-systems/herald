SELECT
  receipts
FROM
  messages
WHERE
  msg_id = ?
LIMIT
  1
