CREATE TABLE IF NOT EXISTS messages (
  id INTEGER PRIMARY KEY,
  author id NOT NULL,
  recipient id NOT NULL,
  timestamp TEXT NOT NULL,
  body TEXT NOT NULL,
  FOREIGN KEY(author) REFERENCES contacts (id),
  FOREIGN KEY(recipient) REFERENCES contacts (id)
)