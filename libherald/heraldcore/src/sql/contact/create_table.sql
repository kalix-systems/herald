CREATE TABLE IF NOT EXISTS contacts (
  -- user id
  user_id TEXT PRIMARY KEY NOT NULL,
  -- name as a string
  name TEXT,
  -- profile picture
  profile_picture TEXT,
  -- Conversation id of the pairwise conversation with the user
  pairwise_conversation BLOB,
  -- user color
  color INTEGER NOT NULL,
  -- Indicates whether contact is archived, defaults to false
  archived INTEGER DEFAULT(0),
  -- Indicates whether a contact is deleted, defaults to false
  deleted INTEGER DEFAULT(0),
  -- Indicates whether the contact is the local user, defaults to false
  local INTEGER DEFAULT(0),
  FOREIGN KEY(pairwise_conversation) REFERENCES conversations(conversation_id)
)