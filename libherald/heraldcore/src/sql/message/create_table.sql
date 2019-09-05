CREATE TABLE IF NOT EXISTS messages (
  id INTEGER PRIMARY KEY,
  author TEXT NOT NULL,
  recipient TEXT NOT NULL,
  timestamp TEXT NOT NULL,
  status INTEGER NOT NULL,
  body TEXT NOT NULL,
  --- message id of message being replied to
  op INTEGER DEFAULT NULL,
  FOREIGN KEY(author) REFERENCES contacts (id),
  FOREIGN KEY(recipient) REFERENCES contacts (id)
)