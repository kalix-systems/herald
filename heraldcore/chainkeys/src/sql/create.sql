CREATE TABLE IF NOT EXISTS ratchet_states(
  conversation_id BLOB NOT NULL,
  -- little-endian u64
  next_ix BLOB NOT NULL,
  base_key BLOB NOT NULL,
  ratchet_key BLOB NOT NULL,
  PRIMARY KEY(conversation_id)
);

CREATE TABLE IF NOT EXISTS derived_keys(
  conversation_id BLOB NOT NULL,
  ix INTEGER NOT NULL,
  msg_key BLOB NOT NULL,
  insertion_ts INTEGER NOT NULL,
  PRIMARY KEY(conversation_id, ix)
);

-- TODO: use this to implement gc strategy
CREATE INDEX IF NOT EXISTS derived_keys_insertion_ts_ix ON derived_keys(insertion_ts);
