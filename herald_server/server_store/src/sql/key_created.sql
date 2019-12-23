SELECT EXISTS (
  SELECT
    1
  FROM
    key_creations
  WHERE
    key_creations.key = $1 AND
  LIMIT 1
)
