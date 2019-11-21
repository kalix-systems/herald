SELECT
    msg_id
FROM
    replies
WHERE
    replies.op_msg_id = @parent_msg_id
