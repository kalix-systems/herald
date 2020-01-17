CREATE TABLE IF NOT EXISTS members (
    conversation_id BLOB NOT NULL,
    user_id TEXT NOT NULL,
    PRIMARY KEY(user_id, conversation_id)
);

CREATE TABLE IF NOT EXISTS ratchets (
    public_key BLOB NOT NULL UNIQUE,
    ratchet BLOB NOT NULL,
    PRIMARY KEY(public_key, ratchet)
);

CREATE TABLE IF NOT EXISTS payloads (
    payload_id BLOB NOT NULL PRIMARY KEY,
    payload BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS pending (
    pending_payload_id BLOB NOT NULL,
    recipient BLOB NOT NULL
);

-- Sigchain tables
CREATE TABLE IF NOT EXISTS sigchain_genesis (
    user_id TEXT NOT NULL PRIMARY KEY,
    ts INTEGER NOT NULL,
    signature BLOB NOT NULL,
    signed_by BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS sigchain_endorsements (
    outer_ts BLOB NOT NULL,
    outer_signature BLOB NOT NULL,
    outer_signed_by BLOB NOT NULL,

    inner_ts BLOB NOT NULL,
    inner_signature BLOB NOT NULL,
    inner_signed_by BLOB NOT NULL,

    user_id TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sigchain_deprecations (
    ts BLOB NOT NULL,
    signature BLOB NOT NULL,
    signed_by BLOB NOT NULL,

    key BLOB NOT NULL,
    user_id TEXT NOT NULL
);
