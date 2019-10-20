SELECT
  keys.key
FROM
  userkeys INNER JOIN keys ON keys.key = userkeys.key
WHERE
  (userkeys.user_id = $1) AND
  (dep_ts IS NULL) AND
  (dep_signed_by IS NULL) AND
  (dep_signature IS NULL)
