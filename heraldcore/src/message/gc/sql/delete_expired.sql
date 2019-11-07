DELETE FROM
  messages
WHERE
  expiration_ts < @time
