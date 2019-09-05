CREATE TABLE IF NOT EXISTS conversation_members (
  member_id TEXT NOT NULL,
  conversation_id TEXT NOT NULL,
  FOREIGN KEY(member_id) REFERENCES contacts(user_id),
  FOREIGN KEY(conversation_id) REFERENCES conversations(conversation_id)
)