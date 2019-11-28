INSERT OR REPLACE INTO
  ratchet_states(conversation_id, public_key, next_ix, base_key, ratchet_key)
VALUES(@cid, @pk, @ix, @base_key, @ratchet_key)
