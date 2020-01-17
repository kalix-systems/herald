INSERT INTO sigchain_endorsements (
    outer_ts,
    outer_signature,
    outer_signed_by,

    inner_ts,
    inner_signature,
    inner_signed_by,

    user_id TEXT
)
VALUES (
    @outer_ts,
    @outer_signature,
    @outer_signed_by,

    @inner_ts,
    @inner_signature,
    @inner_signed_by,

    @user_id
);
