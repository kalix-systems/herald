UPDATE
  messages
SET
  author = NULL,
  body = NULL,
  expiration_ts = NULL,
  insertion_ts = NULL,
  server_ts = NULL,
  send_status = 0,
  has_attachments = 0,
  known = 0
WHERE
  msg_id = ?
