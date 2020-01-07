SELECT
    msg_id
FROM
    messages
WHERE
    conversation_id = @conversation_id
ORDER BY
    insertion_ts DESC
LIMIT 1
