INSERT INTO sigchain_deprecations(
    ts,
    signature,
    signed_by,
    key,
    user_id
)
VALUES(
    @ts,
    @signature,
    @signed_by,
    @key,
    @user_id
);
