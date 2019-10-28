SELECT
  user_id, MAX(receipt_status)
FROM
  read_receipts
WHERE
  msg_id = ?
GROUP BY
  read_receipts.user_id
LIMIT
  1
