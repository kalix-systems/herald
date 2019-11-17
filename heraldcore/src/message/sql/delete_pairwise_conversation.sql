DELETE FROM
  messages
WHERE
  conversation_id IN (
    SELECT
      pairwise_conversation
    FROM
      users
    WHERE
      user_id = @user_id
  );
