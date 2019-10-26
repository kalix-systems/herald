DELETE FROM
  pushes
WHERE
  push_id
IN (
  SELECT
    pending.push_id
  FROM
    pending INNER JOIN pushes ON pending.push_id = pushes.push_id
  WHERE
    pending.key = $1
  ORDER BY
    push_ts ASC, push_id ASC
  LIMIT {limit}
)
