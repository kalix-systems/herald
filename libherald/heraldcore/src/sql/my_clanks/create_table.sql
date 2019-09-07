CREATE TABLE IF NOT EXISTS my_clanks(
  -- associated conversation
  conversation_id BLOB PRIMARY KEY,
  -- KdfKey
  ratchet BLOB NOT NULL,
  -- KxPubKey
  pubkey BLOB NOT NULL,
  -- KxSecKey
  seckey BLOB NOT NULL,
  FOREIGN KEY(conversation_id) REFERENCES conversations(conversation_id)
)