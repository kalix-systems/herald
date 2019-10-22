SELECT EXISTS (
  SELECT
    1
  FROM
    keys
  WHERE
    key = $1 AND
    dep_ts IS NULL AND
    dep_signed_by IS NULL AND
    dep_signature IS NULL
)
