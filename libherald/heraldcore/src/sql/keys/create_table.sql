CREATE TABLE IF NOT EXISTS keys(
  chainkey BLOB NOT NULL,
  hash BLOB NOT NULL,
  used INT NOT NULL DEFAULT(0) CHECK (
    (used = 0)
    OR (used = 1)
  ),
  PRIMARY KEY(chainkey, hash)
)