SELECT
    reactionary,
    react_content,
    insertion_ts
FROM
    message_reactions
WHERE
    msg_id = @msg_id
