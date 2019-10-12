CREATE TABLE IF NOT EXISTS chainkeys(
  chainkey BLOB NOT NULL,
  hash BLOB NOT NULL,
  used INTEGER NOT NULL DEFAULT(0),
  conversation_id BLOB NOT NULL,
  FOREIGN KEY(conversation_id) REFERENCES conversations(conversation_id),
  PRIMARY KEY(chainkey, hash)
);

CREATE TABLE IF NOT EXISTS pending_blocks(
  block_id INTEGER PRIMARY KEY NOT NULL,
  block BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS block_dependencies(
  block_id INTEGER NOT NULL,
  parent_hash BLOB NOT NULL,
  FOREIGN KEY(block_id) REFERENCES pending_blocks(block_id),
  PRIMARY KEY(block_id, parent_hash)
);

CREATE INDEX IF NOT EXISTS block_dep_parent ON block_dependencies(parent_hash);

CREATE TABLE IF NOT EXISTS messages (
  -- message id
  msg_id BLOB PRIMARY KEY NOT NULL,
  -- id of message author
  author TEXT NOT NULL,
  -- id of conversation
  conversation_id BLOB NOT NULL,
  -- text of message
  body TEXT NOT NULL,
  -- timestamp associated with message
  ts INTEGER NOT NULL,
  -- time when message self-destructs
  expiration_date TEXT DEFAULT NULL,
  -- send status of the message
  send_status INTEGER NOT NULL,
  -- read receipts as a map from user ids to receipt status, encoded as CBOR
  receipts BLOB DEFAULT NULL,
  FOREIGN KEY(conversation_id) REFERENCES conversations(conversation_id),
  FOREIGN KEY(author) REFERENCES contacts(user_id)
);

CREATE TABLE IF NOT EXISTS replies (
  -- message id
  msg_id BLOB PRIMARY KEY NOT NULL,
  -- message id of message being replied to
  op_msg_id INTEGER,
  FOREIGN KEY(msg_id) REFERENCES messages(msg_id) ON DELETE CASCADE,
  FOREIGN KEY(op_msg_id) REFERENCES messages(msg_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS reply_op ON replies(op_msg_id);

CREATE TABLE IF NOT EXISTS msg_attachments (
  -- path to media attachment?
  attachment TEXT NOT NULL,
  msg_id BLOB NOT NULL,
  -- TODO this is touchy
  FOREIGN KEY(msg_id) REFERENCES messages(msg_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS contacts (
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
  -- contact status
  status INTEGER NOT NULL,
  -- contact type
  contact_type INTEGER NOT NULL,
  FOREIGN KEY(pairwise_conversation) REFERENCES conversations(conversation_id)
);

CREATE TABLE IF NOT EXISTS user_keys (
  key BLOB NOT NULL PRIMARY KEY,
  user_id BLOB NOT NULL,
  FOREIGN KEY(user_id) REFERENCES contacts(user_id)
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
  -- enforce this table having no more than one row
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
  FOREIGN KEY(member_id) REFERENCES contacts(user_id),
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
  pairwise INTEGER DEFAULT(0)
);
