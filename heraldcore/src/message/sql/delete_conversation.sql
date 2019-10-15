UPDATE
  messages
SET
  author = NULL,
  body = '',
  ts = NULL,
  send_status = 0,
  receipts = NULL,
  expiration_date = NULL,
  receipts = NULL,
  known = 0
WHERE
  conversation_id = ?
