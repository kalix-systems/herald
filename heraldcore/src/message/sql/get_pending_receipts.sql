SELECT
  recipient, MAX(status)
FROM
  pending_receipts
WHERE
  msg_id = ?
GROUP BY
  msg_id, status

