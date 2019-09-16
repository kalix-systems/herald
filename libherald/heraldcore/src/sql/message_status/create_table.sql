CREATE TABLE IF NOT EXISTS message_status(
  msg_id BLOB NOT NULL,
  conversation_id BLOB NOT NULL,
  status INT DEFAULT 0,
  PRIMARY KEY(msg_id, conversation_id),
  FOREIGN KEY(msg_id) REFERENCES messages(msg_id),
  FOREIGN KEY(conversation_id) REFERENCES conversations(conversation_id)
)