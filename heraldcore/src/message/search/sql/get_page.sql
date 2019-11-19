SELECT
    msg_id,
    author,
    conversation_id,
    body,
    insertion_ts,
    has_attachments,
    rowid
FROM
    messages
WHERE
    messages.body IS NOT NULL AND
    messages.insertion_ts < @old_min_time AND
    messages.rowid < @old_row_id
ORDER BY
    messages.insertion_ts DESC,
    -- this is to avoid duplication from our timestamp
    -- being too low resolution
    messages.rowid DESC
LIMIT 1000
