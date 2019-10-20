SELECT
  userkeys.user_id
FROM
  keys INNER JOIN userkeys ON userkeys.key = keys.key
WHERE
  user_id = $1 AND
  dep_signature IS NULL AND
  dep_signed_by IS NULL AND
  dep_ts IS NULL
