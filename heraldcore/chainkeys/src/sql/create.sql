CREATE TABLE IF NOT EXISTS ratchet_states(
  conversation_id BLOB NOT NULL,
  public_key BLOB NOT NULL,
  -- u32
  generation INTEGER NOT NULL,
  -- boolean
  deprecated INTEGER NOT NULL,
  -- little-endian u64
  next_ix BLOB NOT NULL,
  base_key BLOB NOT NULL,
  ratchet_key BLOB NOT NULL,
  PRIMARY KEY(conversation_id, public_key, generation)
);

CREATE INDEX IF NOT EXISTS ratchet_generation_ix ON ratchet_states(generation);

CREATE TABLE IF NOT EXISTS derived_keys(
  conversation_id BLOB NOT NULL,
  public_key BLOB NOT NULL,
  -- u32
  generation INTEGER NOT NULL,
  -- little-endian u64
  ix BLOB NOT NULL,
  msg_key BLOB NOT NULL,
  insertion_ts INTEGER NOT NULL,
  PRIMARY KEY(conversation_id, public_key, generation, ix),
  FOREIGN KEY
    (conversation_id, public_key, generation) 
  REFERENCES ratchet_states
    (conversation_id, public_key, generation)
  ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS derived_keys_generation_ix ON derived_keys(generation);

-- TODO: use this to implement gc strategy
CREATE INDEX IF NOT EXISTS derived_keys_insertion_ts_ix ON derived_keys(insertion_ts);
