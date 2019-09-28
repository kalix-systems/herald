DELETE FROM
  message_status
WHERE
  msg_id
IN (
  SELECT
    msg_id
  FROM
    messages
  WHERE
    messages.conversation_id = ?
  )
