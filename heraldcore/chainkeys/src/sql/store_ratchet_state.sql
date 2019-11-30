INSERT OR REPLACE INTO
  ratchet_states
  ( conversation_id
  , public_key
  , generation
  , next_ix
  , base_key
  , ratchet_key
  , deprecated
  )
VALUES
  ( @cid
  , @pk
  , @gen
  , @ix
  , @base_key
  , @ratchet_key
  , 0
  )
