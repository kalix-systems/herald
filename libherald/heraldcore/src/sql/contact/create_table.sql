CREATE TABLE IF NOT EXISTS contacts (
  -- user id
  user_id TEXT PRIMARY KEY NOT NULL CHECK (length(user_id <= 256)),
  -- name as a string
  name TEXT,
  -- profile picture
  profile_picture TEXT,
  -- Conversation id of the pairwise conversation with the user
  pairwise_conversation BLOB NOT NULL,
  -- user color
  color INTEGER NOT NULL,
  -- contact status
  status INTEGER NOT NULL,
  -- contact type, defaults to false
  contact_type INTEGER NOT NULL,
  FOREIGN KEY(pairwise_conversation) REFERENCES conversations(conversation_id)
)