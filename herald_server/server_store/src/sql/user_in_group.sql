SELECT EXISTS (
    SELECT
        1
    FROM
        conversation_members
    WHERE
        conversation_id = $1 AND
        user_id = $2
    LIMIT 1
)
