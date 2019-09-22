CREATE TABLE IF NOT EXISTS prekeys (
  -- signing_key, 32 bytes
  signing_key BIT(256) PRIMARY KEY,
  -- sealing_key, 32 bytes
  sealing_key BIT(256) NOT NULL,
  FOREIGN KEY(signing_key) REFERENCES creations(signing_key)
)