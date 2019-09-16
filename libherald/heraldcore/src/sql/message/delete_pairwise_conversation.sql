DELETE FROM
  messages
WHERE
  conversation_id IN (
    SELECT
      pairwise_conversation
    FROM
      contacts
    WHERE
      user_id = ?
  );