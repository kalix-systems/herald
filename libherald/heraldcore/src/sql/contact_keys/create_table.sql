CREATE TABLE IF NOT EXISTS contact_keys (
  -- one of the user's public keys
  public_key BLOB PRIMARY KEY NOT NULL,
  -- signature verifying the public key
  signature BLOB NOT NULL,
  -- time the public key was signed
  created INT NOT NULL,
  -- public key of the signer
  signed_by BLOB NOT NULL,
  -- user id the key is associated with
  user_id BLOB NOT NULL,
  /* DEPRECATION INFORMATION */
  -- deprectation status
  -- 0 => deprecated
  -- 1 => active
  status INT DEFAULT(1) NOT NULL,
  -- public key associated with the deprectation
  dep_key BLOB DEFAULT NULL CHECK(
    (status = 1)
    OR (dep_key IS NOT NULL)
  ),
  -- signature verifying the deprecation
  dep_signature BLOB DEFAULT NULL CHECK(
    (status = 1)
    OR (dep_signature IS NOT NULL)
  ),
  -- time the key was deprecated
  dep_time INT DEFAULT NULL CHECK(
    (status = 1)
    OR (dep_time IS NOT NULL)
  )
)