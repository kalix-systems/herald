REPLACE INTO
  derived_keys(conversation_id, public_key, generation, ix, msg_key, insertion_ts)
VALUES(@cid, @pk, @gen, @ix, @msg_key, @ts)
