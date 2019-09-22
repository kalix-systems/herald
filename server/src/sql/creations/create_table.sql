CREATE TABLE IF NOT EXISTS creations (
  -- key, 32 bytes
  key BIT(256) PRIMARY KEY,
  -- signed by, 32 bytes
  signed_by BIT(256) NOT NULL,
  -- timestamp of creation
  creation_ts TIMESTAMP NOT NULL,
  -- signature, 64 bytes
  signature BIT(512) NOT NULL
)
