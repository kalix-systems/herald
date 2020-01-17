SELECT
    ts,
    signature,
    signed_by
FROM
    sigchain_genesis
WHERE
    user_id = @user_id
LIMIT 1
