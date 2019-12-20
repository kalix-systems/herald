SELECT
    user_id
FROM
    userkeys
WHERE
    user_id = $1
LIMIT 1
