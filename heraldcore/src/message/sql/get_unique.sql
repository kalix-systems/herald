SELECT
  hash_dir
FROM
  msg_attachments
WHERE
  msg_id = @msg_id AND
  (SELECT COUNT(DISTINCT hash_dir) FROM msg_attachments) = 1
