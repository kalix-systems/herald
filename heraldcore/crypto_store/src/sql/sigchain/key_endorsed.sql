SELECT EXISTS (
    SELECT
        1
    FROM
        sigchain_endorsements
    WHERE
        inner_signed_by = @key AND
        user_id = @user_id
    LIMIT 1
)
