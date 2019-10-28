SELECT
  hash_dir as hd
FROM
  msg_attachments
WHERE
  msg_id = @msg_id AND
  (SELECT COUNT(hash_dir) FROM msg_attachments WHERE hash_dir = hd)  = 1
