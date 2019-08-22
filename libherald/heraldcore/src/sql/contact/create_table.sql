CREATE TABLE IF NOT EXISTS contacts (
  -- user id
  id TEXT PRIMARY KEY NOT NULL,
  -- name as a string
  name TEXT,
  -- profile picture
  profile_picture TEXT,
  -- user color
  color INTEGER NOT NULL,
  -- Indicates whether contact is archived, defaults to false
  archived INTEGER DEFAULT(0)
)