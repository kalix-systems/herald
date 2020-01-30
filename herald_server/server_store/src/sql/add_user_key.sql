INSERT INTO
  userkeys(user_id, key)
VALUES($1, $2)
ON CONFLICT(key) DO NOTHING;
