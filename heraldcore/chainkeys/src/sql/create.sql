CREATE TABLE IF NOT EXISTS chainkeys(
  chainkey BLOB NOT NULL,
  hash BLOB NOT NULL,
  used INTEGER NOT NULL DEFAULT(0),
  conversation_id BLOB NOT NULL,
  PRIMARY KEY(chainkey, hash)
);

CREATE TABLE IF NOT EXISTS pending_blocks(
  block_id INTEGER PRIMARY KEY NOT NULL,
  global_id_bytes BLOB NOT NULL,
  block BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS block_dependencies(
  block_id INTEGER NOT NULL,
  parent_hash BLOB NOT NULL,
  FOREIGN KEY(block_id) REFERENCES pending_blocks(block_id),
  PRIMARY KEY(block_id, parent_hash)
);

CREATE TABLE IF NOT EXISTS channel_keys(
  conversation_id BLOB NOT NULL,
  channel_key BLOB NOT NULL,
  PRIMARY KEY(conversation_id)
);

CREATE INDEX IF NOT EXISTS block_dep_parent ON block_dependencies(parent_hash);
