INSERT INTO
  pushes(push_data, push_tag, push_ts)
VALUES($1, $2, $3)
RETURNING pushes.push_id
