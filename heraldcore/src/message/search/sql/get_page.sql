SELECT
    msg_id,
    author,
    conversation_id,
    body,
    insertion_ts,
    has_attachments
FROM
    messages
WHERE
    messages.body IS NOT NULL AND
    messages.insertion_ts < @old_min
ORDER BY
    messages.insertion_ts DESC
LIMIT 1000
