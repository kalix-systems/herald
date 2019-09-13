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
  status INTEGER,
  -- indicates whether the contact is local, defaults to false
  local INTEGER DEFAULT(0),
  FOREIGN KEY(pairwise_conversation) REFERENCES conversations(conversation_id),
  -- check status bounds
  CHECK (
    (
      status >= 0
      AND status <= 2
    )
    OR status = NULL
  ),
  -- check that local contact doesn't have status
  CHECK (
    (
      local = 1
      AND status = NULL
    )
    OR (
      local != 1
      AND status != NULL
    )
  )
)