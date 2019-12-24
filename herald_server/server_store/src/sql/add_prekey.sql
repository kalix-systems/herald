INSERT INTO
  prekeys(key, signed_by, signature, ts, slot)
VALUES($1, $2, $3, $4, $5)
ON CONFLICT(key, signed_by) DO NOTHING
