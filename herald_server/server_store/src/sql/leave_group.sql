DELETE FROM
    conversation_members
WHERE
    user_id = $1 AND
    conversation_id = $2
