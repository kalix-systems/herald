SELECT EXISTS (
    SELECT
        1
    FROM
        sigchain_deprecations
    WHERE
        key = @key AND
        user_id = @user_id
    LIMIT 1
)
