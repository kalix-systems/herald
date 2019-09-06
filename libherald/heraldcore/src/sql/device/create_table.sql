CREATE TABLE IF NOT EXISTS devices(
  -- Device id, specific to user
  device_id INT NOT NULL,
  -- User id, globally unique, enforced by server
  user_id INT NOT NULL,
  -- Public key for this device
  signing_key BLOB NOT NULL,
  activation_id INT NOT NULL,
  deprecation_id INT,
  PRIMARY KEY(user_id, device_id),
  FOREIGN KEY(activation_id) REFERENCES activations(activation_id),
  FOREIGN KEY(deactivation_id) REFERENCES deactivations(deactivation_id)
)