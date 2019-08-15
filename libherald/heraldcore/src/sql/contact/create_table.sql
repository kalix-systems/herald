CREATE TABLE IF NOT EXISTS contacts (
  id INTEGER PRIMARY KEY,
  name TEXT,
  archived INTEGER DEFAULT 0 -- Indicates whether contact is archived, defaults to false
)