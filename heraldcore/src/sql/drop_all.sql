-- drop indices
DROP INDEX IF EXISTS msg_id_receipt_ix;
DROP INDEX IF EXISTS reply_op;
DROP INDEX IF EXISTS hash_dir_ix;
DROP INDEX IF EXISTS expiration_ts_ix;
-- drop tables
DROP TABLE IF EXISTS msg_attachments;
DROP TABLE IF EXISTS replies;
DROP TABLE IF EXISTS read_receipts;
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS pending_out;
DROP TABLE IF EXISTS conversation_members;
DROP TABLE IF EXISTS key_creations;
DROP TABLE IF EXISTS key_deprecations;
DROP TABLE IF EXISTS user_keys;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS config;
DROP TABLE IF EXISTS conversations;
DROP INDEX IF EXISTS reply_op;
