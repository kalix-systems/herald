SELECT
    conversation_members.conversation_id
FROM
    conversation_members
INNER JOIN
    conversations
ON
    conversations.conversation_id = conversation_members.conversation_id
WHERE
    member_id = @user_id AND
    conversation_members.conversation_id != @pairwise_cid
ORDER BY
    conversations.last_active_ts DESC
