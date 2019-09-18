SELECT
  used
FROM
  keys
WHERE
  hash = @1
LIMIT
  1