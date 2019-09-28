SELECT
  used
FROM
  chainkeys
WHERE
  hash = @1
LIMIT
  1