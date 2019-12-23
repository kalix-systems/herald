SELECT
    user_id
FROM
    userkeys
WHERE
    key = $1
LIMIT 1
