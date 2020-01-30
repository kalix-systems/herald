SELECT
    sigchain.key,
    inner_signature,
    inner_ts,
    outer_signed_by,
    outer_signature,
    outer_ts,
    update_id
FROM
    sigchain
INNER JOIN
    userkeys
ON
    sigchain.key = userkeys.key
WHERE
    userkeys.user_id = $1
ORDER BY
    sigchain.update_id ASC, sigchain.ts ASC
