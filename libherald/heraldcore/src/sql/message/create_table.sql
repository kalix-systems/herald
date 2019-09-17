CREATE TABLE IF NOT EXISTS messages (
  -- message id
  msg_id BLOB PRIMARY KEY,
  -- id of message author
  author TEXT NOT NULL,
  -- id of conversation
  conversation_id BLOB NOT NULL,
  -- text of message
  body TEXT NOT NULL,
  -- attachment to the message TODO this is another table
  attachment TEXT,
  -- message id of message being replied to
  op_msg_id INT,
  -- timestamp associated with message
  timestamp INT NOT NULL,
  -- time when message self-destructs
  expiration_date TEXT DEFAULT NULL,
  -- send status of the message
  send_status INT,
  FOREIGN KEY(conversation_id) REFERENCES conversations(conversation_id),
  FOREIGN KEY(author) REFERENCES contacts(user_id)
)