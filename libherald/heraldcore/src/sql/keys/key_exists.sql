SELECT
  chainkey
FROM
  keys
WHERE
  hash = @1
  AND used = 0
LIMIT
  1