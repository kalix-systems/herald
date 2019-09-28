SELECT
  name
FROM
  sqlite_master
WHERE
  type = 'table'
  AND name = 'conversation_members'