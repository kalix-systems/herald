SELECT
  messages.insertion_ts,
FROM
  messages
WHERE
  messages.msg_id = @msg_id
