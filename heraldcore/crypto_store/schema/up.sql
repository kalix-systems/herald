CREATE TABLE IF NOT EXISTS members (
   conversation_id BLOB NOT NULL,
   user_id TEXT NOT NULL,
   PRIMARY KEY(user_id, conversation_id)
);

CREATE TABLE IF NOT EXISTS ratchets (
    public_key BLOB NOT NULL,
    ratchet BLOB NOT NULL,
    PRIMARY KEY(public_key, ratchet)
);
