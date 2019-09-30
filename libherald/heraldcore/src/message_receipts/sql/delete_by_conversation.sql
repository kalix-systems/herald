DELETE FROM
  message_receipts
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
