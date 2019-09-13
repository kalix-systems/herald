CREATE TABLE IF NOT EXISTS conversations (
  -- ID of the conversation. Approximately unique.
  conversation_id BLOB PRIMARY KEY NOT NULL CHECK (length(conversation_id) = 32),
  -- Group title, optional
  title TEXT DEFAULT NULL,
  -- Group picture, optional
  picture TEXT DEFAULT NULL,
  -- Group color, default set using hash of id.
  color INTEGER NOT NULL,
  -- Indicates whether conversation is muted, defaults to false
  muted INTEGER DEFAULT(0)
)