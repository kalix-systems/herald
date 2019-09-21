CREATE TABLE IF NOT EXISTS userkeys (
  user_id CHAR(32) NOT NULL,
  signing_key BIT(256) NOT NULL,
  deprecation BIT(256),
  PRIMARY KEY(user_id, signing_key),
  FOREIGN KEY(signing_key) REFERENCES creations(signing_key)
)