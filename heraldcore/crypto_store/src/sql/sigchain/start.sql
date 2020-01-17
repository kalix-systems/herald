INSERT OR IGNORE INTO sigchain_genesis(
    user_id,
    ts,
    signature,
    signed_by
)
VALUES(
    @user_id,
    @ts,
    @signature,
    @signed_by
);
