-- TODO delete associated read receipts
UPDATE
  messages
SET
  author = NULL,
  body = '',
  ts = NULL,
  send_status = 0,
  expiration_date = NULL,
  known = 0
WHERE
  conversation_id IN (
    SELECT
      pairwise_conversation
    FROM
      contacts
    WHERE
      user_id = ?
  );
