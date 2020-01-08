INSERT OR IGNORE INTO
  read_receipts(msg_id, user_id, receipt_status)
VALUES(@1, @2, @3)
