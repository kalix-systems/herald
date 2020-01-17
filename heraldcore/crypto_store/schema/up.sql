CREATE TABLE IF NOT EXISTS members (
   conversation_id BLOB NOT NULL,
   user_id TEXT NOT NULL,
   PRIMARY KEY(user_id, conversation_id)
);
