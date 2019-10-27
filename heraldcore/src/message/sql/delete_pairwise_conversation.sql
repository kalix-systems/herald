-- TODO delete associated read receipts
UPDATE
  messages
SET
  author = NULL,
  body = NULL,
  expiration_ts = NULL,
  insertion_ts = 0,
  server_ts = NULL,
  send_status = 0,
  expiration_ts = NULL,
  known = 0
WHERE
  conversation_id IN (
    SELECT
      pairwise_conversation
    FROM
      contacts
    WHERE
      user_id = @user_id
  );
