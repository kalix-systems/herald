CREATE TABLE IF NOT EXISTS messages (
  -- message id
  msg_id BLOB PRIMARY KEY NOT NULL,
  -- id of message author
  author TEXT,
  -- id of conversation
  conversation_id BLOB NOT NULL,
  -- text of message
  body TEXT,
  -- timestamp associated with message
  insertion_ts INTEGER NOT NULL,
  -- timestamp the message was inserted into the database
  server_ts INTEGER DEFAULT NULL,
  -- time when message self-destructs
  expiration_ts INTEGER DEFAULT NULL,
  -- send status of the message
  send_status INTEGER NOT NULL DEFAULT(0),
  -- is the message a reply?
  is_reply INTEGER NOT NULL DEFAULT(0),
  FOREIGN KEY(conversation_id) REFERENCES conversations(conversation_id),
  FOREIGN KEY(author) REFERENCES users(user_id)
);

CREATE INDEX IF NOT EXISTS expiration_ts_ix ON messages(expiration_ts);

CREATE TABLE IF NOT EXISTS read_receipts (
  -- message id receipt is associated with
  msg_id BLOB NOT NULL,
  -- user id of the user that sent the receipt
  user_id TEXT NOT NULL,
  -- type of receipt that was sent
  receipt_status INTEGER NOT NULL,
  FOREIGN KEY(msg_id) REFERENCES messages(msg_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS msg_id_receipt_ix ON read_receipts(msg_id);

CREATE TABLE IF NOT EXISTS message_reactions (
  -- message id reacts is associated with
  msg_id BLOB NOT NULL,
  -- user id of the user that sent the reacts
  reactionary TEXT NOT NULL,
  -- text of the reacts
  react_content TEXT NOT NULL,
  -- time react was received
  insertion_ts TEXT NOT NULL,
  FOREIGN KEY(msg_id) REFERENCES messages(msg_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS msg_id_react_ix ON read_receipts(msg_id);

CREATE TABLE IF NOT EXISTS replies (
  -- message id
  msg_id BLOB PRIMARY KEY NOT NULL,
  -- message id of message being replied to
  op_msg_id BLOB,
  FOREIGN KEY(msg_id) REFERENCES messages(msg_id) ON DELETE CASCADE,
  FOREIGN KEY(op_msg_id) REFERENCES messages(msg_id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS reply_op ON replies(op_msg_id);

CREATE TABLE IF NOT EXISTS msg_attachments (
  -- path to attachment
  hash_dir TEXT NOT NULL,
  -- number of the attachment
  pos INTEGER NOT NULL,
  msg_id BLOB,
  FOREIGN KEY(msg_id) REFERENCES messages(msg_id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS hash_dir_ix on msg_attachments(hash_dir);

CREATE TABLE IF NOT EXISTS users (
  -- user id
  user_id TEXT PRIMARY KEY NOT NULL,
  -- name as a string
  name TEXT NOT NULL,
  -- profile picture
  profile_picture TEXT,
  -- Conversation id of the pairwise conversation with the user
  pairwise_conversation BLOB NOT NULL,
  -- user color
  color INTEGER NOT NULL,
  -- user status
  status INTEGER NOT NULL,
  -- user type
  user_type INTEGER NOT NULL,
  FOREIGN KEY(pairwise_conversation) REFERENCES conversations(conversation_id)
);

CREATE TABLE IF NOT EXISTS user_keys (
  key BLOB NOT NULL PRIMARY KEY,
  user_id BLOB NOT NULL,
  FOREIGN KEY(user_id) REFERENCES users(user_id)
);

CREATE TABLE IF NOT EXISTS key_creations (
  -- key
  key BLOB PRIMARY KEY NOT NULL,
  -- signing key
  signed_by BLOB NOT NULL,
  -- signature, 64 bytes
  signature BLOB NOT NULL,
  -- timestamp of creation
  ts INTEGER NOT NULL,
  deprecation BLOB DEFAULT NULL,
  FOREIGN KEY(signed_by) REFERENCES user_keys(key),
  FOREIGN KEY(key) REFERENCES user_keys(key)
);

CREATE TABLE IF NOT EXISTS key_deprecations (
  -- key
  key BLOB PRIMARY KEY NOT NULL,
  -- signing key
  signed_by BLOB NOT NULL,
  -- signature, 64 bytes
  signature BLOB NOT NULL,
  -- timestamp of creation
  ts INTEGER NOT NULL,
  FOREIGN KEY(signed_by) REFERENCES user_keys(key),
  FOREIGN KEY(key) REFERENCES user_keys(key)
);

CREATE TABLE IF NOT EXISTS config (
  -- current user id
  id TEXT PRIMARY KEY NOT NULL,
  -- colorscheme
  colorscheme INTEGER NOT NULL,
  kp BLOB NOT NULL,
  -- default preferred expiration period
  preferred_expiration INTEGER DEFAULT NULL,
  -- Address of the server the account is registered on
  home_server BLOB NOT NULL,
  -- enforce this table having no more than one row (for now)
  chk_id INTEGER UNIQUE default(1),
  CONSTRAINT CHK_config_singlerow CHECK (chk_id = 1)
);

CREATE TABLE IF NOT EXISTS pending_out (
  pending_tag INTEGER PRIMARY KEY NOT NULL,
  conversation_id BLOB NOT NULL,
  content BLOB NOT NULL,
  FOREIGN KEY(conversation_id) REFERENCES conversations(conversation_id)
);

CREATE TABLE IF NOT EXISTS conversation_members (
  member_id TEXT NOT NULL,
  conversation_id BLOB NOT NULL,
  FOREIGN KEY(member_id) REFERENCES users(user_id),
  FOREIGN KEY(conversation_id) REFERENCES conversations(conversation_id)
);

CREATE TABLE IF NOT EXISTS conversations (
  -- ID of the conversation.
  conversation_id BLOB PRIMARY KEY NOT NULL,
  -- Group title, optional
  title TEXT DEFAULT NULL,
  -- Group picture, optional
  picture TEXT DEFAULT NULL,
  -- Group color, default set using hash of id.
  color INTEGER NOT NULL,
  -- Indicates whether conversation is muted, defaults to false
  muted INTEGER DEFAULT(0),
  -- Indicates whether a conversation is a canonical pairwise conversation, defaults to false
  pairwise INTEGER DEFAULT(0),
  -- Duration in milliseconds until a message in this conversation expires.
  expiration_period INTEGER DEFAULT NULL,
  -- Time of last important activity
  last_active_ts INTEGER NOT NULL
);
