CREATE TABLE IF NOT EXISTS deprecations (
  -- key, 32 bytes
  key BIT(256) PRIMARY KEY,
  -- timestamp of the deprecation
  deprecation_ts TIMESTAMP NOT NULL,
  signature BIT(512) NOT NULL,
  FOREIGN KEY(signing_key) REFERENCES creations(signing_key)
)
