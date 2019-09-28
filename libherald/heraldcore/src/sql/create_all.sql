CREATE TABLE IF NOT EXISTS chainkeys(
  chainkey BLOB NOT NULL,
  hash BLOB NOT NULL,
  used INTEGER NOT NULL DEFAULT(0),
  PRIMARY KEY(chainkey, hash)
);

CREATE TABLE IF NOT EXISTS message_status(
  msg_id BLOB NOT NULL,
  status INTEGER DEFAULT(0) NOT NULL,
  PRIMARY KEY(msg_id),
  FOREIGN KEY(msg_id) REFERENCES messages(msg_id)
);

CREATE TABLE IF NOT EXISTS messages (
  -- message id
  msg_id BLOB PRIMARY KEY,
  -- id of message author
  author TEXT NOT NULL,
  -- id of conversation
  conversation_id BLOB NOT NULL,
  -- text of message
  body TEXT NOT NULL,
  -- attachment to the message TODO this is another table
  attachment BLOB,
  -- message id of message being replied to
  op_msg_id INTEGER,
  -- timestamp associated with message
  timestamp INTEGER NOT NULL,
  -- time when message self-destructs
  expiration_date TEXT DEFAULT NULL,
  -- send status of the message
  send_status INTEGER,
  FOREIGN KEY(conversation_id) REFERENCES conversations(conversation_id),
  FOREIGN KEY(author) REFERENCES contacts(user_id)
);

CREATE TABLE IF NOT EXISTS contacts (
  -- user id
  user_id TEXT PRIMARY KEY NOT NULL,
  -- name as a string
  name TEXT,
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
  added INTEGER DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY(pairwise_conversation) REFERENCES conversations(conversation_id)
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
