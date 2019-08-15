CREATE TABLE IF NOT EXISTS contacts (
  uid INTEGER PRIMARY KEY,
  name TEXT,
  archived INTEGER DEFAULT 0 -- Indicates whether contact is archived, defaults to false
)