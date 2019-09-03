UPDATE messages
SET send_status=?3
WHERE ID=?2 AND (author=?1 or recipient=?1);

  