UPDATE derived_keys
  SET used = 1
WHERE
  conversation_id = @1,
  AND ix = @2
  
