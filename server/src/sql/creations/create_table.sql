CREATE TABLE IF NOT EXISTS creations (
  -- signing key, 32 bytes
  signing_key BIT(256) PRIMARY KEY,
  -- signed by, 32 bytes
  signed_by BIT(256) NOT NULL,
  -- timestamp of creation
  creation_ts TIMESTAMP NOT NULL,
  -- signature, 64 bytes
  signature BIT(512) NOT NULL
)