SELECT
    conversation_id
FROM
    conversation_members
WHERE
    member_id = @user_id
