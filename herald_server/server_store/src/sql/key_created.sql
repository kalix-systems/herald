SELECT EXISTS (
  SELECT
    1
  FROM
    sigchain
  WHERE
    sigchain.key = $1 AND
    sigchain.is_creation = true
  LIMIT 1
)
