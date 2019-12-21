SELECT
    key_deprecations.key,
    signed_by,
    signature,
    ts
FROM
    key_deprecations
INNER JOIN
    userkeys
ON
    key_deprecations.key = userkeys.key
WHERE
    userkeys.user_id = $1
