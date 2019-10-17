UPDATE
  messages
SET
  author = NULL,
  body = NULL,
  ts = NULL,
  send_status = 0,
  receipts = NULL,
  expiration_date = NULL,
  receipts = NULL,
  has_attachments = 0,
  known = 0
WHERE
  msg_id = ?
