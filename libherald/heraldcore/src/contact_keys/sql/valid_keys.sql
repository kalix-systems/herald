SELECT
  user_keys.key
FROM
  user_keys
INNER JOIN
  key_creations
ON
  (user_keys.key = key_creations.key)
WHERE
  (user_keys.user_id = @1)
EXCEPT
  SELECT
    user_keys.key
  FROM
    user_keys
  INNER JOIN
    key_deprecations
  ON
    (user_keys.key = key_deprecations.key)
  WHERE
    (user_keys.user_id = @1)
