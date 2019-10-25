UPDATE
  messages
SET
  author = NULL,
  body = NULL,
  ts = NULL,
  send_status = 0,
  expiration_date = NULL,
  has_attachments = 0,
  known = 0
WHERE
  conversation_id = ?
