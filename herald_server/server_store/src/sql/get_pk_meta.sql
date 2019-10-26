SELECT
  signed_by, signature, ts, dep_signed_by, dep_signature, dep_ts
FROM
  keys
WHERE
  key = $1
