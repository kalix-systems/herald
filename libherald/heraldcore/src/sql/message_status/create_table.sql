CREATE TABLE IF NOT EXISTS message_status(
  msg_id BLOB NOT NULL,
  user_id BLOB NOT NULL,
  status INT NOT NULL,
  PRIMARY KEY(msg_id, user_id),
  FOREIGN KEY(msg_id) REFERENCES messages(msg_id),
  FOREIGN KEY(user_id) REFERENCES contacts(user_id)
)