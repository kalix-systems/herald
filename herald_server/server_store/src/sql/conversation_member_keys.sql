SELECT
    userkeys.user_id
FROM
    conversation_members
INNER JOIN ON
    conversation_members.user_id = userkeys.user_id
WHERE
    conversation_members.conversation_id = $1
