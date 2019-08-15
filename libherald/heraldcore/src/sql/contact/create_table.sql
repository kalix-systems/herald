CREATE TABLE IF NOT EXISTS conversations (
  author TEXT NOT NULL,
  recipient TEXT NOT NULL,
  timestamp TEXT NOT NULL,
  message TEXT NOT NULL,
  FOREIGN KEY(author) REFERENCES contacts (name),
  FOREIGN KEY(recipient) REFERENCES contacts (name)
)