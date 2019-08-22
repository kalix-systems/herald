DELETE FROM
  messages
WHERE
  author = @1
  OR recipient = @1