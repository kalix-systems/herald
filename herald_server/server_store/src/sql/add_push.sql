INSERT INTO
  pushes(push_data, push_ts)
VALUES($1, $2)
RETURNING pushes.push_id
