INSERT INTO
  sigchain(outer_ts, outer_signed_by, outer_signature, key, is_creation)
VALUES($1, $2, $3, $4, false)
ON CONFLICT(key, is_creation) DO NOTHING
