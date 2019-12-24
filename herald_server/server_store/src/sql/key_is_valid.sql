SELECT EXISTS (
  SELECT
    1
  FROM
    sigchain
  WHERE
    sigchain.key = $1 AND
    sigchain.is_creation = true AND
    sigchain.key NOT IN (
        SELECT
            sigchain.key
        FROM
            sigchain
        WHERE
            is_creation <> false
    )
  LIMIT 1
)
