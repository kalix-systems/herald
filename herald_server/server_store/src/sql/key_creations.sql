SELECT
    inner_signed_by,
    inner_signature,
    inner_ts,
    signed_by,
    signature,
    ts
FROM
    key_creations
INNER JOIN
    userkeys
ON
    key_creations.key = userkeys.key
WHERE
    userkeys.user_id = $1
ORDER BY
    key_creations.ts ASC
