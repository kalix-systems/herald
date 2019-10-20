DELETE FROM prekeys
WHERE sealed_key = any(array(
    SELECT
      sealed_key
    FROM
      prekeys
    WHERE
      signing_key = $1
    LIMIT 1
))
RETURNING sealed_key
