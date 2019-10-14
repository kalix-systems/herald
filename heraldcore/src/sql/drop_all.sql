-- drop indices
DROP INDEX IF EXISTS block_dep_parent;
DROP INDEX IF EXISTS reply_op;
-- drop tables
DROP TABLE IF EXISTS block_dependencies;
DROP TABLE IF EXISTS chainkeys;
DROP TABLE IF EXISTS pending_blocks;
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS pending_receipts;
DROP TABLE IF EXISTS pending_out;
DROP TABLE IF EXISTS replies;
DROP TABLE IF EXISTS conversation_members;
DROP TABLE IF EXISTS key_creations;
DROP TABLE IF EXISTS key_deprecations;
DROP TABLE IF EXISTS user_keys;
DROP TABLE IF EXISTS contacts;
DROP TABLE IF EXISTS config;
DROP TABLE IF EXISTS conversations;
DROP INDEX IF EXISTS reply_op;
