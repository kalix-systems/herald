DELETE FROM
    message_reactions
WHERE
    msg_id = @msg_id AND
    react_content = @react_content AND
    reactionary = @reactionary
