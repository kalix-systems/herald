INSERT INTO
    conversation_members(conversation_id, user_id)
VALUES($1, $2)
ON CONFLICT (conversation_id, user_id) DO NOTHING;
