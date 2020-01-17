SELECT
    ts,
    signature,
    signed_by,
    key
FROM
    sigchain_deprecations
WHERE
    user_id = @user_id
