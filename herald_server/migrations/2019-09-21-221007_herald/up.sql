CREATE TABLE keys (
  -- key, 32 bytes
  key BYTEA PRIMARY KEY,
  -- signed by, 32 bytes
  signed_by BYTEA NOT NULL,
  -- timestamp of creation
  ts TIMESTAMPTZ NOT NULL,
  -- signature, 64 bytes
  signature BYTEA NOT NULL,
  -- timestamp of the deprecation
  dep_ts TIMESTAMPTZ DEFAULT NULL,
  dep_signed_by BYTEA DEFAULT NULL,
  dep_signature BYTEA DEFAULT NULL
);

CREATE TABLE pushes (
  push_id BIGSERIAL PRIMARY KEY,
  push_ts TIMESTAMPTZ NOT NULL,
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
  -- sealing_key, 32 bytes
  sealed_key BYTEA PRIMARY KEY,
  -- signing_key, 32 bytes
  signing_key BYTEA NOT NULL,
  FOREIGN KEY(signing_key) REFERENCES keys(key)
);

CREATE INDEX prekey_signer ON prekeys(signing_key);

CREATE TABLE userkeys (
  user_id CHAR(32) NOT NULL,
  key BYTEA NOT NULL,
  PRIMARY KEY(user_id, key),
  FOREIGN KEY(key) REFERENCES keys(key)
);

