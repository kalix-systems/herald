UPDATE
  keys
SET
  dep_ts = $1,
  dep_signed_by = $2,
  dep_signature = $3
WHERE
  key = $4 AND
  dep_ts IS NULL AND
  dep_signature IS NULL AND
  dep_signed_by IS NULL

