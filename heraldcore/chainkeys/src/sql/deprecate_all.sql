UPDATE OR IGNORE ratchet_states
  SET deprecated = 1
WHERE
  conversation_id = @cid
  AND public_key = @pk
