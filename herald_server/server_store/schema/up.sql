CREATE TABLE userkeys (
    key       BYTEA    NOT NULL PRIMARY KEY,
    user_id   TEXT     NOT NULL
);

CREATE INDEX user_id_ix ON userkeys(user_id);

CREATE TABLE sigchain (
    key               BYTEA     NOT NULL,
    is_creation       BOOLEAN   NOT NULL,
    update_id         BIGSERIAL NOT NULL,

    inner_signature   BYTEA,
    inner_ts          BIGINT,

    outer_signed_by   BYTEA,
    outer_signature   BYTEA,
    outer_ts          BIGINT,

    PRIMARY KEY(key, is_creation)
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
