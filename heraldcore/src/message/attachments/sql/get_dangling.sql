SELECT
  hash_dir
FROM
  msg_attachments
WHERE
  msg_id IS NULL
EXCEPT
SELECT DISTINCT
  hash_dir
FROM
  msg_attachments
WHERE
  msg_id IS NOT NULL
