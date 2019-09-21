CREATE TABLE IF NOT EXISTS prekeys (
  -- signing_key, 32 bytes
  signing_key BIT(256) NOT NULL,
  push_id BIGINT NOT NULL,
  FOREIGN KEY(signing_key) REFERENCES creations(signing_key),
  FOREIGN KEY(push_id) REFERENCES pushes(id)
)