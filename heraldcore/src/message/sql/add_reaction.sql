INSERT OR IGNORE INTO message_reactions(msg_id, reactionary, react_content, insertion_ts)
VALUES(@msg_id, @reactionary, @react_content, @insertion_ts)
