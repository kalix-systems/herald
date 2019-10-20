SELECT
  push_data
FROM
  pushes INNER JOIN pending ON pushes.push_id = pending.push_id
WHERE
  pending.key = $1
ORDER BY
  push_ts ASC, push_id ASC
LIMIT {limit}
