INSERT INTO
  pushes(push_data, push_tag, push_ts, push_user_id, push_key)
VALUES($1, $2, $3, $4, $5)
RETURNING pushes.push_id
