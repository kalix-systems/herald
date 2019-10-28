SELECT
  keys.key,
  keys.signed_by,
  keys.ts,
  keys.signature,
  keys.dep_ts,
  keys.dep_signed_by,
  keys.dep_signature
FROM
  userkeys INNER JOIN keys ON userkeys.key = keys.key
WHERE
  userkeys.user_id = $1
