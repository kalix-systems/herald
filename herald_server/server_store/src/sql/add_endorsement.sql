INSERT INTO
   sigchain(
    key,
    inner_signature,
    inner_ts,
    outer_signed_by,
    outer_signature,
    outer_ts,
    is_creation
   )
VALUES($1, $2, $3, $4, $5, $6, true)
-- ON CONFLICT(key) DO NOTHING;
