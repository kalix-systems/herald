SELECT EXISTS (
  SELECT
    1
  FROM
    key_deprecations
  WHERE
    key_deprecations.key = $1 AND
  LIMIT 1
)
