DELETE FROM
  pending
WHERE
  key = $1 and
  push_id = $2
