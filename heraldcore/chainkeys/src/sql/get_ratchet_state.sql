SELECT
  (next_ix, base_key, ratchet_key)
FROM
  ratchet_states
WHERE
  conversation_id = ?
