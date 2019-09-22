CREATE TABLE keys (
  -- key, 32 bytes
  key BIT(256) PRIMARY KEY,
  -- signed by, 32 bytes
  signed_by BIT(256) NOT NULL,
  -- timestamp of creation
  creation_ts TIMESTAMPTZ NOT NULL,
  -- signature, 64 bytes
  signature BIT(512) NOT NULL,
  -- timestamp of the deprecation
  deprecation_ts TIMESTAMPTZ DEFAULT NULL,
  dep_signed_by BIT(256) DEFAULT NULL,
  dep_signature BIT(512) DEFAULT NULL
);

CREATE TABLE pushes (
  push_id BIGSERIAL PRIMARY KEY,
  push_data BYTEA NOT NULL
);

CREATE TABLE pending (
  key BIT(256) NOT NULL,
  push_id BIGINT NOT NULL,
  PRIMARY KEY(key, push_id),
  FOREIGN KEY(key) REFERENCES keys(key),
  FOREIGN KEY(push_id) REFERENCES pushes(push_id) ON DELETE CASCADE
);

CREATE TABLE prekeys (
  -- signing_key, 32 bytes
  signing_key BIT(256) PRIMARY KEY,
  -- sealing_key, 32 bytes
  sealed_key BYTEA NOT NULL,
  FOREIGN KEY(signing_key) REFERENCES keys(key)
);

CREATE TABLE userkeys (
  user_id CHAR(32) NOT NULL,
  key BIT(256) NOT NULL,
  PRIMARY KEY(user_id, key),
  FOREIGN KEY(key) REFERENCES keys(key)
);

