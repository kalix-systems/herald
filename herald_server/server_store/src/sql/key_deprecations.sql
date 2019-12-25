SELECT
    sigchain.key,
    outer_signed_by,
    outer_signature,
    outer_ts
FROM
    sigchain
INNER JOIN
    userkeys
ON
    sigchain.key = userkeys.key
WHERE
    userkeys.user_id = $1 AND
    sigchain.is_creation = false
