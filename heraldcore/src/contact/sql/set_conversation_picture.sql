UPDATE
    conversations
SET
    picture = @picture
WHERE
conversation_id IN (
    SELECT
        pairwise_conversation
    FROM
        contacts
    WHERE
        user_id = @user_id
    LIMIT 1
)
