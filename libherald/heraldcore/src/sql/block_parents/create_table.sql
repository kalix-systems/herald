CREATE TABLE IF NOT EXISTS block_parents (
  parent_hash BLOB NOT NULL,
  signature BLOB NOT NULL,
  PRIMARY KEY(parent_hash, signature),
  FOREIGN KEY(signature) REFERENCES blocks(sig)
)