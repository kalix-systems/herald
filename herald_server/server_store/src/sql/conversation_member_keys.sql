SELECT
    userkeys.key
FROM
    conversation_members
INNER JOIN
    userkeys
ON
    conversation_members.user_id = userkeys.user_id
WHERE
    conversation_members.conversation_id = $1
