CREATE TABLE IF NOT EXISTS contact_clanks (
  ratchet BLOB NOT NULL,
  pubkey BLOB NOT NULL,
  conversation_id BLOB NOT NULL,
  PRIMARY KEY(ratchet, pubkey),
  FOREIGN KEY(conversation_id) REFERENCES conversations(conversation_id)
)