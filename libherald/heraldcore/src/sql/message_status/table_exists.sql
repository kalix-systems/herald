SELECT
  name
FROM
  sqlite_master
WHERE
  type = 'table'
  AND name = 'message_status'