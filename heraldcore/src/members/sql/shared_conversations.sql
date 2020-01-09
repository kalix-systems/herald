SELECT
    conversation_id
FROM
    conversation_members
WHERE
    member_id = @user_id
AND
    conversation_id != @pairwise_cid
