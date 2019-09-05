CREATE TABLE IF NOT EXISTS messages (
  -- message id
  msg_id INTEGER PRIMARY KEY,
  -- id of message author
  author TEXT NOT NULL,
  -- id of conversation
  conversation TEXT NOT NULL,
  -- timestamp associated with message
  timestamp TEXT NOT NULL,
  -- read and delivery status of message
  status INTEGER NOT NULL,
  -- body of message
  body TEXT NOT NULL,
  -- message id of message being replied to
  op INTEGER DEFAULT NULL,
  FOREIGN KEY(conversation) REFERENCES conversations(conversation_id),
  FOREIGN KEY(author) REFERENCES contacts(contact_id)
)