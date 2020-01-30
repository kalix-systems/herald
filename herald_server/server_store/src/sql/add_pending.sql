INSERT INTO
  pending(key, push_id)
VALUES($1, $2)
ON CONFLICT (key, push_id) DO NOTHING;
