SELECT
  userkeys.user_id
FROM
  userkeys INNER JOIN keys ON userkeys.key = keys.key
WHERE
  userkeys.key = $1 AND
  keys.dep_signature IS NULL AND
  keys.dep_signed_by IS NULL AND
  keys.dep_ts IS NULL
