SELECT
    msg_id
FROM
    replies
WHERE
    op_msg_id = @msg_id
