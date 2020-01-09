SELECT
    msg_id
FROM
    messages
WHERE
    conversation_id = @conversation_id AND
    (messages.expiration_ts IS NULL OR messages.expiration_ts > @current_time)
ORDER BY
    insertion_ts DESC
LIMIT 1
