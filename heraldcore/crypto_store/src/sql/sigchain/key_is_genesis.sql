SELECT EXISTS (
    SELECT
        1
    FROM
        sigchain_genesis
    WHERE
        user_id = @user_id AND
        signed_by = @key
    LIMIT 1
)
