SELECT
    key,
    slot,
    signed_by,
    signature,
    ts
FROM
    prekeys
WHERE
    signed_by = $1
ORDER BY RANDOM()
LIMIT 1
