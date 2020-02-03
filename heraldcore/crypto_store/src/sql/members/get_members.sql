SELECT
    user_id
FROM
    members
WHERE
    conversation_id = @conversation_id
