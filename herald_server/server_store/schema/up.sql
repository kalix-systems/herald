CREATE TABLE key_creations (
  -- key, 32 bytes
  key BYTEA PRIMARY KEY,
  -- signed by, 32 bytes
  signed_by BYTEA NOT NULL,
  -- timestamp of creation
  ts BIGINT NOT NULL,
  -- signature, 64 bytes
  signature BYTEA NOT NULL
);

CREATE TABLE key_deprecations (
  -- key, 32 bytes
  key BYTEA PRIMARY KEY,
  -- timestamp of the deprecation
  ts BIGINT NOT NULL,
  signed_by BYTEA NOT NULL,
  signature BYTEA NOT NULL,
  FOREIGN KEY(key) REFERENCES key_creations(key)
);

CREATE TABLE pushes (
  push_id BIGSERIAL PRIMARY KEY,
  push_ts BIGINT NOT NULL,
  push_data BYTEA NOT NULL
);

CREATE INDEX push_ts_ix ON pushes(push_ts);

CREATE TABLE pending (
  key BYTEA NOT NULL,
  push_id BIGINT NOT NULL,
  PRIMARY KEY(key, push_id),
  FOREIGN KEY(key) REFERENCES keys(key),
  FOREIGN KEY(push_id) REFERENCES pushes(push_id) ON DELETE CASCADE
);

CREATE TABLE prekeys (
  -- sealed_key, 32 bytes
  sealed_key BYTEA NOT NULL,
  -- signing_key, 32 bytes
  signing_key BYTEA NOT NULL,
  key_number INT2 NOT NULL,
  FOREIGN KEY(signing_key) REFERENCES keys(key)
);

CREATE INDEX prekey_signer ON prekeys(signing_key);

CREATE TABLE userkeys (
  user_id CHAR(32) NOT NULL,
  key BYTEA NOT NULL,
  PRIMARY KEY(user_id, key),
  FOREIGN KEY(key) REFERENCES key_creations(key)
);

CREATE INDEX userkey_key_ix ON userkeys(key);

CREATE TABLE conversation_members (
    conversation_id BYTEA NOT NULL,
    user_id CHAR(32) NOT NULL,
    FOREIGN KEY(conversation_id) REFERENCES(conversations.conversation_id),
    FOREIGN KEY(user_id) REFERENCES(userkeys.user_id)
);
