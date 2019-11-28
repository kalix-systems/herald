REPLACE INTO
  derived_keys(conversation_id, public_key, ix, msg_key, insertion_ts)
VALUES(@cid, @pk, @ix, @msg_key, @ts)
