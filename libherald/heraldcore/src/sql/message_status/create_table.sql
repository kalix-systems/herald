CREATE TABLE IF NOT EXISTS message_status(
  msg_id BLOB NOT NULL,
  user_id BLOB NOT NULL,
  status INT NOT NULL DEFAULT(0)
)