SELECT
  push_data, push_tag, push_ts, pending.push_id as push_id
FROM
  pushes INNER JOIN pending ON pushes.push_id = pending.push_id
WHERE
  pending.key = $1
ORDER BY
  push_ts ASC, pending.push_id ASC
