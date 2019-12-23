CREATE TABLE userkeys (
    user_id   TEXT     NOT NULL,
    key       BYTEA    NOT NULL PRIMARY KEY
);

CREATE INDEX user_id_ix ON userkeys(user_id);

CREATE TABLE key_creations (
    key               BYTEA     PRIMARY KEY,
    inner_signature   BYTEA     NOT NULL,
    inner_ts          BIGINT    NOT NULL,

    signed_by         BYTEA,
    signature         BYTEA,
    ts                BIGINT
);

CREATE TABLE key_deprecations (
    key         BYTEA    PRIMARY KEY,
    ts          BIGINT   NOT NULL,
    signed_by   BYTEA    NOT NULL,
    signature   BYTEA    NOT NULL
);


CREATE TABLE pushes (
    push_id     BIGSERIAL   PRIMARY   KEY,
    push_ts     BIGINT      NOT NULL,
    push_data   BYTEA       NOT NULL,
    push_tag    BYTEA       NOT NULL
);

CREATE INDEX push_ts_ix ON pushes(push_ts);

CREATE TABLE pending (
    key       BYTEA    NOT NULL,
    push_id   BIGINT   NOT NULL,

    PRIMARY KEY(key, push_id)
);

CREATE TABLE prekeys (
    sealed_key    BYTEA   NOT NULL,
    signing_key   BYTEA   NOT NULL
);

CREATE INDEX prekey_signer ON prekeys(signing_key);

CREATE TABLE conversation_members (
    conversation_id   BYTEA  NOT NULL,
    user_id           TEXT   NOT NULL,
    PRIMARY KEY(conversation_id, user_id)
);
