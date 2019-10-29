SELECT
  hash_dir
FROM
  msg_attachments
WHERE
  msg_id = ?
ORDER BY
  msg_attachments.pos ASC
