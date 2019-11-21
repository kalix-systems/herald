SELECT
  global_id_bytes, block
FROM
  pending_blocks
WHERE
  pending_blocks.block_id
NOT IN
  (SELECT block_id FROM block_dependencies)
ORDER BY
  pending_blocks.block_id ASC
