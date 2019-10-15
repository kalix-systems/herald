DELETE FROM
   pending_blocks
WHERE
  pending_blocks.block_id
NOT IN
  (SELECT block_id FROM block_dependencies)
