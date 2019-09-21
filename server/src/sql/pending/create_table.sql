CREATE TABLE IF NOT EXISTS pending (
  signing_key BIT(256) NOT NULL,
  push_id BIGINT NOT NULL,
  FOREIGN KEY(signing_key) REFERENCES creations(signing_key),
  FOREIGN KEY(push_id) REFERENCES pushes(id)
)