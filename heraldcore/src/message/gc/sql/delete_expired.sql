DELETE FROM
  messages
WHERE
  expired_ts < @time
